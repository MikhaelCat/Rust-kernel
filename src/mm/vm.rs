//! Virtual Memory Area (VMA) Management
//! 
//! Управление виртуальной памятью процессов

use super::error::MmError;

/// Тип памяти в VMA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmaType {
    /// Обычное mmap
    Anonymous,
    /// Файловое отображение
    FileMapping,
    /// Stack
    Stack,
    /// Heap
    Heap,
    /// Shared memory
    Shared,
}

/// Права доступа к странице
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VmaFlags(u32);

impl VmaFlags {
    pub const NONE: u32 = 0;
    pub const READ: u32 = 1 << 0;   // Чтение
    pub const WRITE: u32 = 1 << 1;  // Запись  
    pub const EXECUTE: u32 = 1 << 2;// Выполнение
    
    pub fn empty() -> Self {
        Self(Self::NONE)
    }
    
    pub fn all() -> Self {
        Self(Self::READ | Self::WRITE | Self::EXECUTE)
    }
    
    pub fn with_read(mut self) -> Self {
        self.0 |= Self::READ;
        self
    }
    
    pub fn with_write(mut self) -> Self {
        self.0 |= Self::WRITE;
        self
    }
    
    pub fn with_execute(mut self) -> Self {
        self.0 |= Self::EXECUTE;
        self
    }
    
    pub fn has(&self, flag: u32) -> bool {
        self.0 & flag != 0
    }
}

/// Карта страниц для отслеживания mapping виртуальных -> физических
#[derive(Debug, Clone)]
pub struct PageMapEntry {
    pub vaddr: usize,          // Виртуальный адрес начала страницы
    pub paddr: usize,          // Физический адрес
    pub flags: u64,            // Flags страницы
    pub reference_count: u32,  // Reference count для shared pages
}

impl PageMapEntry {
    pub fn new(vaddr: usize, paddr: usize, flags: u64) -> Self {
        Self {
            vaddr,
            paddr,
            flags,
            reference_count: 1,
        }
    }
    
    pub fn increment_ref(&mut self) {
        self.reference_count += 1;
    }
    
    pub fn decrement_ref(&mut self) -> bool {
        if self.reference_count > 0 {
            self.reference_count -= 1;
            self.reference_count == 0
        } else {
            false
        }
    }
}

/// Virtual Memory Area - область виртуальной памяти процесса
#[derive(Debug, Clone)]
pub struct Vma {
    pub vm_start: usize,        // Начальный адрес области
    pub vm_end: usize,          // Конечный адрес области
    pub vm_flags: VmaFlags,     // Флаги области
    pub vm_type: VmaType,       // Тип области
    pub vm_offset: usize,       // Offset в файле для file mappings
    pub vm_file: Option<String>, // Ссылка на файл (если файловое отображение)
    pub page_map: Vec<PageMapEntry>, // Карта страниц внутри этой области
    pub next: Option<Box<Vma>>,  // Следующий VMA в списке
    pub prev: Option<Box<Vma>>,  // Предыдущий VMA
}

impl Default for Vma {
    fn default() -> Self {
        Self::new(0, 0, VmaFlags::empty(), VmaType::Anonymous)
    }
}

impl Vma {
    pub fn new(start: usize, end: usize, flags: VmaFlags, vma_type: VmaType) -> Self {
        Self {
            vm_start: start,
            vm_end: end,
            vm_flags: flags,
            vm_type: vma_type,
            vm_offset: 0,
            vm_file: None,
            page_map: Vec::new(),
            next: None,
            prev: None,
        }
    }
    
    /// Создать новый VMA с флагами Read/Write
    pub fn anon_vma(start: usize, len: usize) -> Self {
        Self::new(start, start + len, VmaFlags::all(), VmaType::Anonymous)
    }
    
    /// Создать стек VMA
    pub fn stack_vma(start: usize, len: usize) -> Self {
        let mut flags = VmaFlags::all();
        flags.0 &= !VmaFlags::EXECUTE; // Stack обычно без execute
        Self::new(start, start + len, flags, VmaType::Stack)
    }
    
    /// Создать файловый маппинг
    pub fn file_vma(start: usize, len: usize, file_path: &str) -> Result<Self, MmError> {
        if file_path.is_empty() {
            return Err(MmError::InvalidParameter);
        }
        
        Ok(Self {
            vm_start: start,
            vm_end: start + len,
            vm_flags: VmaFlags::with_read().with_write(),
            vm_type: VmaType::FileMapping,
            vm_offset: 0,
            vm_file: Some(file_path.to_string()),
            page_map: Vec::new(),
            next: None,
            prev: None,
        })
    }
    
    /// Длина области
    pub fn len(&self) -> usize {
        self.vm_end - self.vm_start
    }
    
    /// Пустая ли область
    pub fn is_empty(&self) -> bool {
        self.vm_start == self.vm_end
    }
    
    /// Добавить страницу в мап
    pub fn add_page_mapping(&mut self, entry: PageMapEntry) -> Result<(), MmError> {
        // Проверка что страница находится в пределах VMA
        if entry.vaddr < self.vm_start || entry.vaddr >= self.vm_end {
            return Err(MmError::InvalidAddress);
        }
        
        self.page_map.push(entry);
        Ok(())
    }
    
    /// Найти страницу по виртуальному адресу
    pub fn find_page(&self, vaddr: usize) -> Option<&PageMapEntry> {
        self.page_map.iter().find(|entry| entry.vaddr == vaddr)
    }
    
    /// Удалить страницу из мапа
    pub fn remove_page_mapping(&mut self, vaddr: usize) -> Result<bool, MmError> {
        if let Some(pos) = self.page_map.iter().position(|e| e.vaddr == vaddr) {
            self.page_map.remove(pos);
            Ok(true)
        } else {
            Err(MmError::InvalidAddress)
        }
    }
    
    /// Разрешен ли доступ чтения
    pub fn can_read(&self) -> bool {
        self.vm_flags.has(VmaFlags::READ)
    }
    
    /// Разрешен ли доступ записи
    pub fn can_write(&self) -> bool {
        self.vm_flags.has(VmaFlags::WRITE)
    }
    
    /// Разрешено ли выполнение
    pub fn can_execute(&self) -> bool {
        self.vm_flags.has(VmaFlags::EXECUTE)
    }
    
    /// Добавить следующий VMA в список
    pub fn link_next(&mut self, next: Vma) {
        self.next = Some(Box::new(next));
        if let Some(ref mut next_vma) = self.next {
            next_vma.prev = Some(Box::new(Vma {
                vm_start: 0, vm_end: 0,
                vm_flags: VmaFlags::empty(), vm_type: VmaType::Anonymous,
                vm_offset: 0, vm_file: None, page_map: Vec::new(),
                next: None, prev: Some(Box::new(Vma {
                    vm_start: 0, vm_end: 0,
                    vm_flags: VmaFlags::empty(), vm_type: VmaType::Anonymous,
                    vm_offset: 0, vm_file: None, page_map: Vec::new(),
                    next: None, prev: None,
                }))
            }));
        }
    }
}

/// Менеджер VMA - хранение всех областей памяти процесса
#[derive(Debug)]
pub struct VmaManager {
    pub root: Option<Box<Vma>>,      // Корневой VMA в списке
    pub total_vmas: usize,           // Количество VMA
    pub total_mapped_bytes: usize,   // Общий объем mapped памяти
}

impl Default for VmaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl VmaManager {
    pub fn new() -> Self {
        Self {
            root: None,
            total_vmas: 0,
            total_mapped_bytes: 0,
        }
    }
    
    /// Добавить новый VMA
    pub fn add_vma(&mut self, vma: Vma) -> Result<(), MmError> {
        if vma.is_empty() {
            return Err(MmError::InvalidParameter);
        }
        
        // Проверка пересечения с существующими VMA
        if let Some(root) = &self.root {
            if self.overlaps_with(root, &vma) {
                return Err(MmError::InvalidParameter);
            }
        }
        
        // Добавляем в конец списка
        let new_vma = Box::new(vma);
        if self.root.is_none() {
            self.root = Some(new_vma);
        } else {
            let mut current = self.root.as_mut().unwrap();
            while let Some(ref mut next) = current.next {
                current = next;
            }
            let prev_vma = std::mem::replace(current, new_vma.clone());
            current.prev = Some(Box::new(prev_vma));
            current.next = Some(Box::new(Vma {
                vm_start: 0, vm_end: 0,
                vm_flags: VmaFlags::empty(), vm_type: VmaType::Anonymous,
                vm_offset: 0, vm_file: None, page_map: Vec::new(),
                next: None, prev: current.prev.clone(),
            }));
        }
        
        self.total_vmas += 1;
        self.total_mapped_bytes += new_vma.len();
        Ok(())
    }
    
    /// Проверить перекрытие двух VMA
    fn overlaps_with(&self, a: &Vma, b: &Vma) -> bool {
        a.vm_start < b.vm_end && b.vm_start < a.vm_end
    }
    
    /// Удалить VMA по адресу начала
    pub fn remove_vma_by_start(&mut self, addr: usize) -> Result<Option<Vma>, MmError> {
        if let Some(pos) = self.find_vma_index(addr) {
            let vma = self.remove_at_index(pos)?;
            self.total_vmas -= 1;
            self.total_mapped_bytes -= vma.len();
            return Ok(Some(vma));
        }
        Err(MmError::NoSuchProcess)
    }
    
    /// Найти индекс VMA по началу адреса
    fn find_vma_index(&self, addr: usize) -> Option<usize> {
        let mut current = &self.root;
        let mut index = 0;
        
        while let Some(vma) = current {
            if vma.vm_start == addr {
                return Some(index);
            }
            index += 1;
            current = &vma.next;
        }
        
        None
    }
    
    /// Найти VMA содержащий адрес
    pub fn find_vma(&self, addr: usize) -> Option<&Vma> {
        let mut current = &self.root;
        
        while let Some(vma) = current {
            if addr >= vma.vm_start && addr < vma.vm_end {
                return Some(vma);
            }
            current = &vma.next;
        }
        
        None
    }
    
    /// Найти VMA mutable
    pub fn find_vma_mut(&mut self, addr: usize) -> Option<&mut Vma> {
        let mut current = &mut self.root;
        
        while let Some(vma) = current {
            if addr >= vma.vm_start && addr < vma.vm_end {
                return Some(vma);
            }
            current = &mut vma.next;
        }
        
        None
    }
    
    /// Удалить VMA по индексу
    fn remove_at_index(&mut self, index: usize) -> Result<Vma, MmError> {
        if index == 0 {
            if let Some(root) = self.root.take() {
                return Ok(*root);
            }
        }
        
        let mut current = self.root.as_mut().unwrap();
        for _ in 0..index-1 {
            current = current.next.as_mut().ok_or(MmError::InvalidAddress)?;
        }
        
        let next = current.next.take().ok_or(MmError::InvalidAddress)?;
        Ok(*next)
    }
    
    /// Итератор по всем VMA
    pub fn iter_vmas(&self) -> VmaIterator<'_> {
        VmaIterator {
            current: self.root.as_deref(),
        }
    }
    
    /// Общее количество байт
    pub fn total_mapped_bytes(&self) -> usize {
        self.total_mapped_bytes
    }
    
    /// Количество VMA
    pub fn vma_count(&self) -> usize {
        self.total_vmas
    }
}

/// Итератор по VMA списку
pub struct VmaIterator<'a> {
    current: Option<&'a Vma>,
}

impl<'a> Iterator for VmaIterator<'a> {
    type Item = &'a Vma;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.next.as_deref();
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vma_creation() {
        let vma = Vma::anon_vma(0x1000, 0x1000);
        assert_eq!(vma.vm_start, 0x1000);
        assert_eq!(vma.vm_end, 0x2000);
        assert!(vma.can_read());
        assert!(vma.can_write());
    }
    
    #[test]
    fn test_vma_manager_add() {
        let mut manager = VmaManager::new();
        let vma = Vma::anon_vma(0x1000, 0x1000);
        
        assert!(manager.add_vma(vma).is_ok());
        assert_eq!(manager.total_vmas, 1);
        assert_eq!(manager.total_mapped_bytes(), 0x1000);
    }
    
    #[test]
    fn test_vma_manager_find() {
        let mut manager = VmaManager::new();
        let vma = Vma::anon_vma(0x1000, 0x1000);
        
        manager.add_vma(vma.clone()).unwrap();
        
        let found = manager.find_vma(0x1500);
        assert!(found.is_some());
        assert_eq!(found.unwrap().vm_start, 0x1000);
        
        let not_found = manager.find_vma(0x5000);
        assert!(not_found.is_none());
    }
    
    #[test]
    fn test_vma_page_mapping() {
        let mut vma = Vma::anon_vma(0x1000, 0x4000);
        let entry = PageMapEntry::new(0x1000, 0x8000, 0);
        
        assert!(vma.add_page_mapping(entry).is_ok());
        assert_eq!(vma.page_map.len(), 1);
        
        let found = vma.find_page(0x1000);
        assert!(found.is_some());
        assert_eq!(found.unwrap().paddr, 0x8000);
    }
}
