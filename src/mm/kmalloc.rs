//! Slab Allocator Implementation for Linux Kernel
//! 
//! Эффективный аллокатор для малых объектов (<4KB)
//! Использует пре-выделенные кэши и per-CPU оптимизации

use super::error::MmError;
use crate::mm::PageFrame;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Размер страницы
const PAGE_SIZE: usize = 4096;

/// Классы slab allocator'а для разных размеров объектов
#[derive(Debug, Clone)]
pub struct SlabClass {
    pub name: String,                // Имя класса (например "kmalloc-64")
    pub object_size: usize,          // Размер одного объекта
    pub objects_per_slab: usize,     // Объектов в одном слое (слэбе)
    pub slab_size: usize,            // Полный размер слэба (страницы * количество)
    pub free_objects: AtomicUsize,   // Количество свободных объектов (атомарно)
    pub slabs_allocated: usize,      // Выделенные слэбы
    pub total_objects: usize,        // Общее кол-во объектов во всех слэбах
}

impl SlabClass {
    pub fn new(name: &str, object_size: usize) -> Self {
        let objects_per_slab = PAGE_SIZE / object_size;
        let slab_size = if objects_per_slab > 0 { objects_per_slab * PAGE_SIZE } else { PAGE_SIZE };
        
        Self {
            name: name.to_string(),
            object_size,
            objects_per_slab,
            slab_size,
            free_objects: AtomicUsize::new(0),
            slabs_allocated: 0,
            total_objects: 0,
        }
    }
    
    /// Рассчитать сколько объектов в одной странице
    pub fn objects_per_page(&self) -> usize {
        self.object_size.min(PAGE_SIZE / self.object_size)
    }
    
    /// Рассчитать размер объекта с выравниванием
    pub fn aligned_object_size(size: usize) -> usize {
        // Выравнивание до 8 байт для производительности
        ((size + 7) / 8) * 8
    }
}

/// Один слой (slab) - набор выделенных страниц
#[derive(Debug)]
pub struct Slab {
    pub pages: Vec<PageFrame>,       // Физические страницы
    pub free_list: Vec<usize>,       // Свободные индексы объектов (free list)
    pub is_full: bool,               // Заполнен ли полностью
    pub is_empty: bool,              // Пустой ли полностью
    pub shared: bool,                // Общий между CPU или приватный
}

impl Default for Slab {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Slab {
    pub fn new(shared: bool) -> Self {
        Self {
            pages: Vec::new(),
            free_list: Vec::new(),
            is_full: false,
            is_empty: true,
            shared,
        }
    }
    
    /// Добавить страницу в слэб
    pub fn add_page(&mut self, page: PageFrame) {
        self.pages.push(page);
    }
    
    /// Инициализировать free list для N страниц
    pub fn init_free_list(&mut self, num_pages: usize, object_size: usize) {
        for page_idx in 0..num_pages {
            let page_start = page_idx * PAGE_SIZE;
            for obj_offset in (0..PAGE_SIZE).step_by(object_size) {
                let offset = page_start + obj_offset;
                if offset < page_idx * PAGE_SIZE + PAGE_SIZE {
                    self.free_list.push(offset);
                }
            }
        }
        self.is_empty = self.free_list.is_empty();
    }
    
    /// Проверить есть ли свободные объекты
    pub fn has_free_objects(&self) -> bool {
        !self.free_list.is_empty()
    }
    
    /// Получить количество свободных объектов
    pub fn free_count(&self) -> usize {
        self.free_list.len()
    }
    
    /// Заполнен ли слэб
    pub fn is_complete(&self) -> bool {
        !self.is_empty && self.free_list.is_empty()
    }
}

/// Перф-CPU кэш для быстрого выделения без блокировок
#[derive(Debug)]
pub struct CpukCache {
    pub cpu_id: usize,
    pub partial_slabs: Vec<Slab>,     // Частично заполненные слэбы
    pub full_slabs: Vec<Slab>,        // Полностью заполненные слэбы  
    pub empty_slabs: Vec<Slab>,       // Пустые слэбы
    pub batch_order: usize,           // Порядок батчинга
    pub count: usize,                 // Текущее кол-во объектов в кэше
    pub high: usize,                  // Пиковое значение
    pub limit: usize,                 // Лимит объектов в кэше
}

impl Default for CpukCache {
    fn default() -> Self {
        Self::new(0)
    }
}

impl CpukCache {
    pub fn new(cpu_id: usize) -> Self {
        let limit = 8 * 1024 / PAGE_SIZE; // Ограничение ~8K страниц
        
        Self {
            cpu_id,
            partial_slabs: Vec::new(),
            full_slabs: Vec::new(),
            empty_slabs: Vec::new(),
            batch_order: 3,
            count: 0,
            high: 0,
            limit,
        }
    }
    
    /// Добавить частичный слэб
    pub fn add_partial(&mut self, slab: Slab) {
        self.partial_slabs.push(slab);
    }
    
    /// Добавить полный слэб
    pub fn add_full(&mut self, slab: Slab) {
        self.full_slabs.push(slab);
    }
    
    /// Добавить пустой слэб
    pub fn add_empty(&mut self, slab: Slab) {
        self.empty_slabs.push(slab);
        self.count += self.limit.max(1);
    }
    
    /// Удалить первый частичный слэб
    pub fn remove_partial(&mut self) -> Option<Slab> {
        self.partial_slabs.pop()
    }
    
    /// Получить последний частичный слэб (LIFO)
    pub fn get_partial(&mut self) -> Option<&mut Slab> {
        self.partial_slabs.last_mut()
    }
    
    /// Обновить пиковое значение
    pub fn update_high(&mut self) {
        if self.count > self.high {
            self.high = self.count;
        }
    }
    
    /// Проверить нужно ли рефоilling
    pub fn should_refill(&self) -> bool {
        self.count <= self.limit / 4
    }
    
    /// Проверить можно ли освободить
    pub fn can_shrink(&self) -> bool {
        self.count >= self.limit / 8
    }
}

/// Slab Allocator основной менеджер
#[derive(Debug)]
pub struct SlabAllocator {
    pub class_slabs: HashMap<usize, SlabClass>, // Кэши по размеру объекта
    pub per_cpu_caches: Vec<CpukCache>,         // Per-CPU кэши
    pub next_class_id: usize,                   // ID следующего класса
    pub total_allocated_bytes: AtomicUsize,     // Всего выделено байт
    pub total_slabs: AtomicUsize,               // Всего слэбов
}

impl Default for SlabAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl SlabAllocator {
    pub fn new() -> Self {
        let mut allocator = Self {
            class_slabs: HashMap::new(),
            per_cpu_caches: vec![CpukCache::new(0)],
            next_class_id: 0,
            total_allocated_bytes: AtomicUsize::new(0),
            total_slabs: AtomicUsize::new(0),
        };
        
        // Создать стандартные классы
        allocator.create_cache("kmalloc-16", 16);
        allocator.create_cache("kmalloc-32", 32);
        allocator.create_cache("kmalloc-64", 64);
        allocator.create_cache("kmalloc-128", 128);
        allocator.create_cache("kmalloc-256", 256);
        allocator.create_cache("kmalloc-512", 512);
        allocator.create_cache("kmalloc-1024", 1024);
        allocator.create_cache("kmalloc-2048", 2048);
        
        allocator
    }
    
    /// Создать новый кэш для определенного размера
    pub fn create_cache(&mut self, name: &str, size: usize) -> Result<(), MmError> {
        if size == 0 || size > 4096 {
            return Err(MmError::InvalidParameter);
        }
        
        let aligned_size = SlabClass::aligned_object_size(size);
        let cache = SlabClass::new(name, aligned_size);
        
        self.class_slabs.insert(aligned_size, cache);
        Ok(())
    }
    
    /// Найти кэш для требуемого размера
    fn find_cache(&self, size: usize) -> Option<(usize, &SlabClass)> {
        let aligned_size = SlabClass::aligned_object_size(size);
        
        // Найти точное совпадение или ближайший больший
        self.class_slabs
            .iter()
            .find(|(_, cache)| cache.object_size >= aligned_size)
            .or_else(|| {
                // Если не найдено, используем самый большой доступный
                self.class_slabs
                    .values()
                    .max_by_key(|c| c.object_size)
                    .map(|cache| (cache.object_size, cache))
            })
    }
    
    /// Find mutable cache by size
    fn find_cache_mut(&mut self, size: usize) -> Option<(usize, &mut SlabClass)> {
        let aligned_size = SlabClass::aligned_object_size(size);
        
        self.class_slabs
            .iter_mut()
            .find(|(_, cache)| cache.object_size >= aligned_size)
            .or_else(|| {
                self.class_slabs
                    .values_mut()
                    .max_by_key(|c| c.object_size)
                    .map(|cache| (cache.object_size, cache))
            })
    }
    
    /// Выделить объект из kmem cache
    pub fn alloc_from_cache(&mut self, cache_size: usize) -> Result<*mut u8, MmError> {
        let (size, cache) = self.find_cache(cache_size)
            .ok_or(MmError::NoMemory)?;
        
        // Найти перф-CPU кэш (используем CPU 0 для простоты)
        let cpu_cache = self.per_cpu_caches.get_mut(0)
            .ok_or(MmError::NoMemory)?;
        
        // Ищем свободный объект в partial слэбах
        if let Some(partial) = cpu_cache.get_partial() {
            if let Some(offset) = partial.free_list.pop() {
                cache.free_objects.fetch_sub(1, Ordering::SeqCst);
                
                // Если слэб стал полным
                if partial.free_list.is_empty() {
                    if let Some(pos) = cpu_cache.partial_slabs.iter().rposition(|s| {
                        s.pages.iter().zip(partial.pages.iter()).all(|(a,b)| a.index == b.index)
                    }) {
                        let slab = cpu_cache.partial_slabs.remove(pos);
                        cpu_cache.full_slabs.push(slab);
                    }
                }
                
                // Вычисляем адрес объекта
                let page = partial.pages.first().ok_or(MmError::NoMemory)?;
                let obj_addr = page.physical_address + offset;
                
                cache.total_objects += 1;
                self.total_allocated_bytes.fetch_add(size as usize, Ordering::SeqCst);
                
                return Ok(obj_addr as *mut u8);
            }
        }
        
        // Нужно выделить новый слэб
        self.alloc_new_slab(cache_size, cache)
    }
    
    /// Выделить новый слэб
    fn alloc_new_slab(&mut self, size: usize, cache: &SlabClass) -> Result<*mut u8, MmError> {
        // Выделяем N страниц для нового слэба
        let num_pages = (cache.slab_size + PAGE_SIZE - 1) / PAGE_SIZE;
        
        let mut new_slab = Slab::new(false);
        
        for _ in 0..num_pages {
            // В реальности здесь был бы вызов physical memory allocator
            // Для демо создаем фейковые страницы
            let fake_page = PageFrame::new(new_slab.pages.len() * PAGE_SIZE / size, PAGE_SIZE);
            new_slab.add_page(fake_page);
        }
        
        // Инициализируем free list
        new_slab.init_free_list(num_pages, cache.object_size);
        cache.free_objects.store(new_slab.free_count(), Ordering::SeqCst);
        
        self.total_slabs.fetch_add(1, Ordering::SeqCst);
        cache.slabs_allocated += 1;
        new_slab.is_empty = false;
        
        // Добавляем в partial слэбы CPU cache
        if let Some(cpu_cache) = self.per_cpu_caches.get_mut(0) {
            cpu_cache.add_partial(new_slab);
            
            // Обновляем счетчики
            let obj_count = cpu_cache.partial_slabs
                .last()
                .map(|s| s.free_count())
                .unwrap_or(0);
            cpu_cache.count += obj_count;
            cpu_cache.update_high();
        }
        
        // Возвращаем первый свободный объект
        let offset = {
            let slab = self.per_cpu_caches.get_mut(0)
                .and_then(|c| c.partial_slabs.last())
                .ok_or(MmError::NoMemory)?;
            slab.free_list.first().copied()
                .ok_or(MmError::NoMemory)?
        };
        
        let page = {
            self.per_cpu_caches.get_mut(0)
                .and_then(|c| c.partial_slabs.last())
                .and_then(|s| s.pages.first())
                .ok_or(MmError::NoMemory)?
        };
        
        let obj_addr = page.physical_address + offset;
        Ok(obj_addr as *mut u8)
    }
    
    /// Освободить объект
    pub fn free_object(&mut self, ptr: *mut u8, size: usize) -> Result<(), MmError> {
        if ptr.is_null() {
            return Err(MmError::InvalidAddress);
        }
        
        let obj_addr = ptr as usize;
        
        // Найти кэш (по размеру)
        let (cache_size, cache) = self.find_cache(size)
            .ok_or(MmError::InvalidParameter)?;
        
        // Найти слэб который содержит этот объект
        let found = self.per_cpu_caches.iter_mut().any(|cpu_cache| {
            // Проверяем partial слэбы
            if let Some(partial) = cpu_cache.get_partial() {
                if let Some(offset) = partial.free_list.iter().position(|&o| {
                    partial.pages.first()
                        .map(|p| p.physical_address + o == obj_addr)
                        .unwrap_or(false)
                }) {
                    // Возвращаем в свободный список
                    partial.free_list.push(offset);
                    cache.free_objects.fetch_add(1, Ordering::SeqCst);
                    
                    // Создаем новый empty слэб если все объекты вернулись
                    if partial.free_list.len() >= partial.free_list.capacity().max(1) {
                        // Логика упрощена для демонстрации
                    }
                    
                    return true;
                }
            }
            false
        });
        
        if !found {
            return Err(MmError::InvalidAddress);
        }
        
        self.total_allocated_bytes.fetch_sub(cache_size, Ordering::SeqCst);
        Ok(())
    }
    
    /// Get statistics
    pub fn stats(&self) -> SlabStats {
        let mut total_free = 0;
        let mut total_objects = 0;
        
        for (_, cache) in &self.class_slabs {
            total_free += cache.free_objects.load(Ordering::SeqCst);
            total_objects += cache.total_objects;
        }
        
        SlabStats {
            total_slabs: self.total_slabs.load(Ordering::SeqCst),
            total_allocated: self.total_allocated_bytes.load(Ordering::SeqCst),
            total_free_objects: total_free,
            total_cached_objects: total_objects,
            cpu_count: self.per_cpu_caches.len(),
        }
    }
    
    /// Add CPU cache
    pub fn add_cpu(&mut self, cpu_id: usize) {
        if !self.per_cpu_caches.iter().any(|c| c.cpu_id == cpu_id) {
            self.per_cpu_caches.push(CpukCache::new(cpu_id));
        }
    }
}

/// Статистика Slab Allocator
#[derive(Debug, Clone)]
pub struct SlabStats {
    pub total_slabs: usize,
    pub total_allocated: usize,
    pub total_free_objects: usize,
    pub total_cached_objects: usize,
    pub cpu_count: usize,
}

/// Public kmalloc API
/// Выделение памяти через slab allocator
#[inline]
pub fn kmalloc(size: usize) -> Result<*mut u8, MmError> {
    // Инициализируем глобальный allocator (в реальном ядре это thread_local)
    static ALLOCATOR: std::sync::OnceLock<std::sync::Mutex<SlabAllocator>> = std::sync::OnceLock::new();
    
    let allocator = ALLOCATOR.get_or_init(|| std::sync::Mutex::new(SlabAllocator::new()));
    let mut guard = allocator.lock().unwrap();
    guard.alloc_from_cache(size)
}

/// Free memory through slab allocator
#[inline]
pub fn kfree(ptr: *mut u8, size: usize) -> Result<(), MmError> {
    static ALLOCATOR: std::sync::OnceLock<std::sync::Mutex<SlabAllocator>> = std::sync::OnceLock::new();
    
    let allocator = ALLOCATOR.get_or_init(|| std::sync::Mutex::new(SlabAllocator::new()));
    let mut guard = allocator.lock().unwrap();
    guard.free_object(ptr, size)
}

/// kmalloc с инициализацией нулями
#[inline]
pub fn kzmalloc(size: usize) -> Result<*mut u8, MmError> {
    let ptr = kmalloc(size)?;
    if !ptr.is_null() {
        unsafe {
            let slice = std::slice::from_raw_parts_mut(ptr, PAGE_SIZE);
            slice.fill(0);
        }
    }
    Ok(ptr)
}

/// Allocate multiple objects
#[inline]
pub fn kmalloc_array(count: usize, size: usize) -> Result<*mut u8, MmError> {
    if count == 0 {
        return Err(MmError::InvalidParameter);
    }
    
    // Проверка на переполнение
    let total_size = count.checked_mul(size)
        .ok_or(MmError::NoMemory)?;
    
    kmalloc(total_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_slab_class_creation() {
        let cache = SlabClass::new("test-cache", 64);
        assert_eq!(cache.name, "test-cache");
        assert_eq!(cache.object_size, 64);
        assert_eq!(cache.objects_per_page(), 64);
    }
    
    #[test]
    fn test_kmalloc_basic() {
        let ptr = kmalloc(64).unwrap();
        assert!(!ptr.is_null());
        
        // Записать данные
        unsafe {
            *(ptr as *mut u32) = 0x12345678;
        }
        
        // Прочитать обратно
        unsafe {
            assert_eq!(*(ptr as *mut u32), 0x12345678);
        }
        
        // Освободить
        kfree(ptr, 64).unwrap();
    }
    
    #[test]
    fn test_kmalloc_different_sizes() {
        let ptr1 = kmalloc(16).unwrap();
        let ptr2 = kmalloc(128).unwrap();
        let ptr3 = kmalloc(1024).unwrap();
        
        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        assert!(!ptr3.is_null());
        
        kfree(ptr1, 16).unwrap();
        kfree(ptr2, 128).unwrap();
        kfree(ptr3, 1024).unwrap();
    }
    
    #[test]
    fn test_kcalloc() {
        let ptr = kzmalloc(256).unwrap();
        assert!(!ptr.is_null());
        
        // Проверить что инициализирован нулями
        unsafe {
            let slice = std::slice::from_raw_parts(ptr, 256);
            assert!(slice.iter().all(|&b| b == 0));
        }
        
        kfree(ptr, 256).unwrap();
    }
    
    #[test]
    fn test_allocator_stats() {
        let _allocator = SlabAllocator::new();
        
        let stats = SlabAllocator::new().stats();
        assert_eq!(stats.cpu_count, 1);
        assert!(stats.total_slabs >= 0);
    }
}
