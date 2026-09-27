//! Swap Space Management for Linux Kernel
//!
//! Управление своп-пространством для page-out/Page-in операций

use super::error::MmError;
use crate::mm::{PageFrame, PhysicalMemoryManager};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

/// Swap area - область на disk для swap
#[derive(Debug, Clone)]
pub struct SwapArea {
    pub device_id: u32,              // ID устройства swap
    pub swap_size: u64,              // Размер swap в pages
    pub used_pages: AtomicU32,       // Использовано страниц
    pub free_pages: u64,             // Свободно страниц
    pub priority: i8,                // Приоритет области
    pub path: String,                // Путь к файлу swap или устройству
}

impl Default for SwapArea {
    fn default() -> Self {
        Self::new(0, 0, "none".to_string(), 5)
    }
}

impl SwapArea {
    pub fn new(device_id: u32, size_in_pages: u64, path: String, priority: i8) -> Self {
        Self {
            device_id,
            swap_size: size_in_pages,
            used_pages: AtomicU32::new(0),
            free_pages: size_in_pages,
            priority,
            path,
        }
    }
    
    /// Выделить swap slot
    pub fn allocate_slot(&self) -> Result<u64, MmError> {
        let used = self.used_pages.load(Ordering::SeqCst);
        if used >= self.swap_size as u32 {
            return Err(MmError::NoMemory);
        }
        
        let slot = used as u64;
        self.used_pages.fetch_add(1, Ordering::SeqCst);
        Ok(slot)
    }
    
    /// Освободить swap slot
    pub fn free_slot(&self, slot: u64) -> Result<(), MmError> {
        let current_used = self.used_pages.load(Ordering::SeqCst);
        if slot >= current_used as u64 {
            return Err(MmError::InvalidAddress);
        }
        
        self.used_pages.fetch_sub(1, Ordering::SeqCst);
        Ok(())
    }
    
    /// Доступно ли место в swap
    pub fn has_space(&self) -> bool {
        let used = self.used_pages.load(Ordering::SeqCst);
        (used as u64) < self.swap_size
    }
    
    /// Сколько свободно места
    pub fn available_space(&self) -> u64 {
        let used = self.used_pages.load(Ordering::SeqCst);
        self.swap_size - (used as u64)
    }
}

/// Swap cache entry - кэш страницы в swap
#[derive(Debug, Clone)]
pub struct SwapCacheEntry {
    pub page_frame_number: u64,      // PFN страницы в physical memory
    pub swap_offset: u64,            // Offset в swap области
    pub device_id: u32,              // Device swap area
    pub reference_count: u32,        // Reference count
    pub dirty: bool,                 // Загрязнена ли страница (требует write-back)
    pub locked: bool,                //Locked (cannot be moved)
}

impl SwapCacheEntry {
    pub fn new(pfn: u64, offset: u64, device_id: u32) -> Self {
        Self {
            page_frame_number: pfn,
            swap_offset: offset,
            device_id,
            reference_count: 1,
            dirty: false,
            locked: false,
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
    
    /// Проверить можно ли удалить entry
    pub fn can_remove(&self) -> bool {
        self.reference_count == 0 && !self.locked && !self.dirty
    }
}

/// Swap preferences и настройки
#[derive(Debug, Clone)]
pub struct SwapPreferences {
    pub swappiness: u8,              // От 0 до 100 - предпочтения swap
    pub max_swap_files: usize,       // Максимум файлов swap
    pub stripe_size: usize,          // Stripe size для RAID
    pub vignette_ratio: u32,         // Ratio для clean/unclean pages
}

impl Default for SwapPreferences {
    fn default() -> Self {
        Self {
            swappiness: 60,              // Standard value
            max_swap_files: 8,
            stripe_size: 64 * 1024,
            vignette_ratio: 100,
        }
    }
}

/// Главный менеджер swap space
#[derive(Debug)]
pub struct SwapManager {
    pub areas: Vec<SwapArea>,                    // Swap области
    pub swap_cache: HashMap<u64, SwapCacheEntry>,// Kэш pages в swap
    pub preferences: SwapPreferences,            // Preferences
    pub total_swap: AtomicU64,                   // Total swap space
    pub used_swap: AtomicU64,                    // Использовано swap
    pub pages_pinned: AtomicU64,                 // Зажатые в memory pages
    pub pages_swapped_in: AtomicU64,             // Статистика
    pub pages_swapped_out: AtomicU64,
}

impl Default for SwapManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SwapManager {
    pub fn new() -> Self {
        Self {
            areas: Vec::new(),
            swap_cache: HashMap::new(),
            preferences: SwapPreferences::default(),
            total_swap: AtomicU64::new(0),
            used_swap: AtomicU64::new(0),
            pages_pinned: AtomicU64::new(0),
            pages_swapped_in: AtomicU64::new(0),
            pages_swapped_out: AtomicU64::new(0),
        }
    }
    
    /// Добавить swap область
    pub fn add_area(&mut self, area: SwapArea) -> Result<(), MmError> {
        if self.areas.len() >= self.preferences.max_swap_files {
            return Err(MmError::NoMemory);
        }
        
        if area.swap_size == 0 || area.path.is_empty() {
            return Err(MmError::InvalidParameter);
        }
        
        self.total_swap.fetch_add(area.swap_size, Ordering::SeqCst);
        self.areas.push(area);
        Ok(())
    }
    
    /// Найти лучшую swap область по приоритету
    fn select_best_area(&self) -> Option<&SwapArea> {
        self.areas.iter().max_by_key(|a| a.priority)
    }
    
    /// Выбрать swap область для page-out
    fn select_area_for_out(&self, page_flags: u64) -> Option<&SwapArea> {
        // Выберем область с самым высоким приоритетом который имеет место
        self.areas.iter()
            .filter(|a| a.has_space())
            .max_by_key(|a| a.priority)
    }
    
    /// Вынести страницу в swap (Page-Out)
    pub fn page_out(&mut self, phys_page: &PageFrame, file_offset: Option<u64>) -> Result<u64, MmError> {
        // Выбор swap области
        let area = self.select_area_for_out(0)
            .ok_or(MmError::NoMemory)?;
        
        // Выделение swap slot
        let swap_offset = area.allocate_slot()?;
        
        // Создание entry в swap cache
        let mut entry = SwapCacheEntry::new(
            phys_page.index as u64,
            swap_offset,
            area.device_id,
        );
        
        entry.dirty = true; // Page modified, needs to be written
        
        // Добавляем в swap cache
        self.swap_cache.insert(phys_page.index as u64, entry);
        
        // Update статистики
        self.used_swap.fetch_add(1, Ordering::SeqCst);
        self.pages_swapped_out.fetch_add(1, Ordering::SeqCst);
        
        println!("Page {} swapped out to offset {} on device {}", 
                 phys_page.index, swap_offset, area.device_id);
        
        Ok(swap_offset)
    }
    
    /// Внести страницу из swap (Page-In)
    pub fn page_in(&mut self, swap_offset: u64, device_id: u32) -> Result<PageFrame, MmError> {
        // Поиск entry в swap cache
        let entry = self.swap_cache.values()
            .find(|e| e.swap_offset == swap_offset && e.device_id == device_id)
            .ok_or(MmError::NoSuchProcess)?;
        
        // Создаем новую physical page
        let page = PageFrame::new(entry.page_frame_number as usize, 4096);
        
        // Decrement reference
        if let Some(cache_entry) = self.swap_cache.get_mut(&page.index as u64) {
            cache_entry.decrement_ref();
        }
        
        // Update статистики
        self.pages_swapped_in.fetch_add(1, Ordering::SeqCst);
        
        println!("Page {} swapped in from offset {} on device {}", 
                 page.index, swap_offset, device_id);
        
        Ok(page)
    }
    
    /// Удалить entry из swap cache
    pub fn remove_from_cache(&mut self, pfn: u64) -> Result<bool, MmError> {
        if let Some(entry) = self.swap_cache.remove(&pfn) {
            if entry.dirty {
                // Нужно записать обратно в swap если dirty
                self.write_back_to_swap(entry.swap_offset, entry.device_id);
            }
            
            // Обновляем счетчики
            if entry.dirty {
                self.used_swap.fetch_sub(1, Ordering::SeqCst);
                
                // Найти area и освободить slot
                if let Some(area) = self.areas.iter_mut().find(|a| a.device_id == entry.device_id) {
                    area.free_slot(entry.swap_offset)?;
                }
            }
            
            return Ok(true);
        }
        
        Err(MmError::InvalidAddress)
    }
    
    /// Write back dirty page to swap
    fn write_back_to_swap(&mut self, swap_offset: u64, device_id: u32) {
        println!("Writing back page to swap: offset={}, device={}", swap_offset, device_id);
        // В реальности здесь был бы I/O вызов для записи страницы
    }
    
    /// Увеличить swappiness
    pub fn increase_swappiness(&mut self, amount: u8) {
        if self.preferences.swappiness + amount <= 100 {
            self.preferences.swappiness += amount;
        } else {
            self.preferences.swappiness = 100;
        }
    }
    
    /// Уменьшить swappiness  
    pub fn decrease_swappiness(&mut self, amount: u8) {
        if self.preferences.swappiness >= amount {
            self.preferences.swappiness -= amount;
        } else {
            self.preferences.swappiness = 0;
        }
    }
    
    /// Получить статистику
    pub fn get_stats(&self) -> SwapStats {
        SwapStats {
            total_swap: self.total_swap.load(Ordering::SeqCst),
            used_swap: self.used_swap.load(Ordering::SeqCst),
            free_swap: self.total_swap.load(Ordering::SeqCst) - self.used_swap.load(Ordering::SeqCst),
            pages_swapped_in: self.pages_swapped_in.load(Ordering::SeqCst),
            pages_swapped_out: self.pages_swapped_out.load(Ordering::SeqCst),
            cached_pages: self.swap_cache.len() as u64,
            pinned_pages: self.pages_pinned.load(Ordering::SeqCst),
            areas_count: self.areas.len() as u64,
            swappiness: self.preferences.swappiness,
        }
    }
    
    /// Проверить нужно ли делать page-out
    pub fn should_swap(&self) -> bool {
        let used = self.used_swap.load(Ordering::SeqCst);
        let total = self.total_swap.load(Ordering::SeqCst);
        
        if total == 0 {
            return false;
        }
        
        // Если используем больше 50% swap и swappiness высокая
        (used as f64 / total as f64) > 0.5 && self.preferences.swappiness > 50
    }
    
    /// Очистить все swap entries
    pub fn clear_all(&mut self) {
        self.swap_cache.clear();
        self.used_swap.store(0, Ordering::SeqCst);
        
        for area in &mut self.areas {
            area.used_pages.store(0, Ordering::SeqCst);
        }
    }
}

/// Статистика Swap
#[derive(Debug, Clone)]
pub struct SwapStats {
    pub total_swap: u64,
    pub used_swap: u64,
    pub free_swap: u64,
    pub pages_swapped_in: u64,
    pub pages_swapped_out: u64,
    pub cached_pages: u64,
    pub pinned_pages: u64,
    pub areas_count: u64,
    pub swappiness: u8,
}

impl SwapStats {
    pub fn utilization(&self) -> f64 {
        if self.total_swap == 0 {
            return 0.0;
        }
        (self.used_swap as f64 / self.total_swap as f64) * 100.0
    }
    
    pub fn throughput(&self) -> f64 {
        self.pages_swapped_in as f64 + self.pages_swapped_out as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swap_area_allocation() {
        let area = SwapArea::new(0, 1000, "/dev/swap1".to_string(), 5);
        
        assert!(area.has_space());
        assert_eq!(area.available_space(), 1000);
        
        let slot = area.allocate_slot().unwrap();
        assert_eq!(slot, 0);
        assert_eq!(area.available_space(), 999);
        
        area.free_slot(0).unwrap();
        assert_eq!(area.available_space(), 1000);
    }
    
    #[test]
    fn test_swap_manager_add_area() {
        let mut manager = SwapManager::new();
        let area = SwapArea::new(0, 1000, "/dev/swap1".to_string(), 5);
        
        assert!(manager.add_area(area).is_ok());
        assert_eq!(manager.total_swap.load(Ordering::SeqCst), 1000);
    }
    
    #[test]
    fn test_page_out_and_in() {
        let mut manager = SwapManager::new();
        
        // Добавляем swap область
        let area = SwapArea::new(0, 1000, "/dev/swap1".to_string(), 5);
        manager.add_area(area).unwrap();
        
        // Выделяем physical страницу (fake)
        let page = PageFrame::new(100, 4096);
        
        // Swapping out
        let offset = manager.page_out(&page, None).unwrap();
        assert!(offset > 0);
        
        // Swapping in
        let restored_page = manager.page_in(offset, 0).unwrap();
        assert_eq!(restored_page.index, page.index);
    }
    
    #[test]
    fn test_swap_statistics() {
        let mut manager = SwapManager::new();
        let stats_before = manager.get_stats();
        
        assert_eq!(stats_before.used_swap, 0);
        
        let area = SwapArea::new(0, 1000, "/dev/swap1".to_string(), 5);
        manager.add_area(area).unwrap();
        
        let page = PageFrame::new(100, 4096);
        manager.page_out(&page, None).unwrap();
        
        let stats_after = manager.get_stats();
        assert_eq!(stats_after.used_swap, 1);
        assert_eq!(stats_after.pages_swapped_out, 1);
    }
}
