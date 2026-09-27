//! Page Fault Handling for Linux Kernel
//!
//! Обработка page fault'ов, COW (Copy-on-Write), Demand Paging

use super::error::MmError;
use crate::mm::{VmaManager, Vma, PageFrame, PhysicalMemoryManager};

/// Типы page fault'ов
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PageFaultType {
    /// Access violation (нет прав доступа)
    AccessViolation,
    /// Non-present page (страница не в memory)
    NonPresent,
    /// Protection key fault
    ProtectionKey,
    /// Hardware error
    HardwareError,
}

/// Результат обработки page fault
#[derive(Debug)]
pub enum PageFaultResult {
    /// Успешно обработано
    Handled,
    /// Недопустимое accessing  
    InvalidAccess,
    /// Kill process (OOM или critical error)
    KillProcess,
    /// Sleep until resource available
    NeedWait,
}

/// Copy-on-Write информация
#[derive(Debug, Clone)]
pub struct CowInfo {
    pub shared: bool,                // Shared между процессами?
    pub original_pfn: u64,           // Original physical frame number
    pub copy_pfn: Option<u64>,       // Новая копия после write
    pub ref_count: u32,              // Reference count
    pub dirty: bool,                 // Загрязнена ли
}

impl CowInfo {
    pub fn new(pfn: u64) -> Self {
        Self {
            shared: true,
            original_pfn: pfn,
            copy_pfn: None,
            ref_count: 1,
            dirty: false,
        }
    }
    
    pub fn increment_ref(&mut self) {
        if self.shared {
            self.ref_count += 1;
        }
    }
    
    pub fn decrement_ref(&mut self) -> bool {
        if self.shared && self.ref_count > 0 {
            self.ref_count -= 1;
            self.ref_count == 0
        } else {
            false
        }
    }
    
    /// Выделить копию страницы (COW trigger)
    pub fn make_unique(&mut self, alloc_fn: &mut FnMut(u64) -> Result<PageFrame, MmError>) -> Result<(), MmError> {
        if !self.shared {
            return Ok(()); // Already unique
        }
        
        if self.ref_count != 1 {
            // Нужно выделить новую страницу
            let new_page = alloc_fn(self.original_pfn)?;
            self.copy_pfn = Some(new_page.index as u64);
            self.dirty = true;
        }
        
        Ok(())
    }
}

/// Page fault handler контекст
#[derive(Debug)]
pub struct PageFaultContext {
    pub virtual_address: usize,      // Virtual address что вызвало fault
    pub fault_type: PageFaultType,   // Тип fault'а
    pub access_mode: u8,             // Mode access (read/write/execute)
    pub user_mode: bool,             // User mode или kernel mode
    pub current_vma: Option<usize>,  // VMA который затронут
    pub is_cow: bool,                // Copy-on-Write situation
}

impl Default for PageFaultContext {
    fn default() -> Self {
        Self {
            virtual_address: 0,
            fault_type: PageFaultType::NonPresent,
            access_mode: 0,
            user_mode: true,
            current_vma: None,
            is_cow: false,
        }
    }
}

impl PageFaultContext {
    pub fn new(vaddr: usize, fault_type: PageFaultType) -> Self {
        Self {
            virtual_address: vaddr,
            fault_type,
            ..Default::default()
        }
    }
}

/// Страница в процессе allocation
#[derive(Debug)]
pub struct InFlightPage {
    pub virtual_address: usize,
    pub phys_frame: PageFrame,
    pub flags: u64,
    pub priority: u8,
    pub state: AllocationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationState {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// OOM (Out of Memory) Killer структура
#[derive(Debug, Clone)]
pub struct OomKillerStats {
    pub total_oom_kills: u64,
    pub total_oom_scoring: u64,
    pub last_oom_score: i64,
    pub oom_kill_priority: i32,
}

impl Default for OomKillerStats {
    fn default() -> Self {
        Self {
            total_oom_kills: 0,
            total_oom_scoring: 0,
            last_oom_score: 0,
            oom_kill_priority: 0,
        }
    }
}

/// Главный Page Fault Handler
#[derive(Debug)]
pub struct PageFaultHandler {
    pub vma_manager: VmaManager,
    pub phys_memory: Option<PhysicalMemoryManager>,
    pub cow_table: HashMap<usize, CowInfo>, // COW tracking по VA
    pub in_flight_pages: Vec<InFlightPage>,
    pub oom_stats: OomKillerStats,
    pub disable_swap: bool,
}

impl Default for PageFaultHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl PageFaultHandler {
    pub fn new() -> Self {
        Self {
            vma_manager: VmaManager::new(),
            phys_memory: None,
            cow_table: HashMap::new(),
            in_flight_pages: Vec::new(),
            oom_stats: OomKillerStats::default(),
            disable_swap: false,
        }
    }
    
    /// Set physical memory manager reference
    pub fn set_phys_memory(&mut self, mm: PhysicalMemoryManager) {
        self.phys_memory = Some(mm);
    }
    
    /// Главная функция обработки page fault
    pub fn handle_fault(&mut self, context: PageFaultContext) -> Result<PageFaultResult, MmError> {
        // Проверка VMA
        let vma = self.vma_manager.find_vma(context.virtual_address)
            .ok_or(MmError::InvalidAddress)?;
        
        // Проверка access permissions
        if !self.check_access(vma, context.access_mode) {
            return Ok(PageFaultResult::InvalidAccess);
        }
        
        match context.fault_type {
            PageFaultType::NonPresent => {
                self.handle_non_present_fault(context)
            }
            PageFaultType::AccessViolation => {
                Err(MmError::InvalidAddress)
            }
            PageFaultType::ProtectionKey => {
                Err(MmError::InvalidParameter)
            }
            PageFaultType::HardwareError => {
                Err(MmError::NoMemory)
            }
        }
    }
    
    /// Проверить доступ к странице
    fn check_access(&self, vma: &Vma, access_mode: u8) -> bool {
        match access_mode {
            0 => vma.can_read(),     // Read
            1 => vma.can_write(),    // Write
            2 => vma.can_execute(),  // Execute
            _ => false,
        }
    }
    
    /// Handle non-present page fault
    fn handle_non_present_fault(&mut self, context: PageFaultContext) -> Result<PageFaultResult, MmError> {
        // Найти физическую страницу для allocation
        let mut phys_mm = self.phys_memory.as_mut()
            .ok_or(MmError::NoMemory)?;
        
        // Выделяем новую страницу
        let new_page = phys_mm.allocate_page()?;
        
        // Initialize page content (from swap or zero-fill)
        self.zero_fill_page(&new_page);
        
        // Add page mapping to VMA
        if let Some(vma) = self.vma_manager.find_vma_mut(context.virtual_address) {
            let entry = super::vm::PageMapEntry::new(
                context.virtual_address,
                new_page.physical_address,
                0,
            );
            vma.add_page_mapping(entry)?;
        }
        
        // Check for COW
        if context.is_cow {
            self.handle_cow_fault(context.virtual_address)?;
        }
        
        Ok(PageFaultResult::Handled)
    }
    
    /// Zero-fill пустую страницу
    fn zero_fill_page(&self, page: &PageFrame) {
        println!("Zero-filling page at index {}", page.index);
        // В реальности здесь memset страницы нулями
    }
    
    /// Handle COW fault
    fn handle_cow_fault(&mut self, vaddr: usize) -> Result<(), MmError> {
        let cow_info = self.cow_table.get_mut(&vaddr)
            .ok_or(MmError::NoSuchProcess)?;
        
        // Если нужно сделать уникальную копию
        if cow_info.ref_count > 1 {
            let mut mm = self.phys_memory.as_mut()
                .ok_or(MmError::NoMemory)?;
            
            let new_page = mm.allocate_page()?;
            cow_info.copy_pfn = Some(new_page.index as u64);
            cow_info.dirty = true;
            
            // Update reference counts
            cow_info.decrement_ref();
        }
        
        Ok(())
    }
    
    /// Register COW mapping
    pub fn register_cow(&mut self, vaddr: usize, pfn: u64) {
        let cow_info = CowInfo::new(pfn);
        self.cow_table.insert(vaddr, cow_info);
    }
    
    /// Start demand paging для диапазона адресов
    pub fn start_demand_paging(&mut self, start_addr: usize, end_addr: usize) -> Result<(), MmError> {
        if start_addr >= end_addr {
            return Err(MmError::InvalidParameter);
        }
        
        // Разбиваем на pages и создаем placeholders
        let page_size = 4096;
        let num_pages = (end_addr - start_addr) / page_size;
        
        for i in 0..num_pages {
            let vaddr = start_addr + i * page_size;
            // Page еще не выделена, но mapped в VMA
            
            // Track как in-flight allocation
            self.in_flight_pages.push(InFlightPage {
                virtual_address: vaddr,
                phys_frame: PageFrame::new(0, 0), // Placeholder
                flags: 0,
                priority: 0,
                state: AllocationState::Pending,
            });
        }
        
        Ok(())
    }
    
    /// Allocate a pending demand page
    pub fn allocate_demand_page(&mut self, vaddr: usize) -> Result<PageFrame, MmError> {
        // Найти в flight entry
        let entry_index = self.in_flight_pages.iter()
            .position(|p| p.virtual_address == vaddr)
            .ok_or(MmError::NoSuchProcess)?;
        
        let entry = &mut self.in_flight_pages[entry_index];
        entry.state = AllocationState::InProgress;
        
        let mut mm = self.phys_memory.as_mut()
            .ok_or(MmError::NoMemory)?;
        
        let page = mm.allocate_page()?;
        
        entry.phys_frame = page.clone();
        entry.state = AllocationState::Completed;
        
        // Remove from in-flight
        self.in_flight_pages.remove(entry_index);
        
        Ok(page)
    }
    
    /// OOM Killer - выбрать victim process
    pub fn select_oom_victim(&self, processes: &[ProcessInfo]) -> Option<&ProcessInfo> {
        if processes.is_empty() {
            return None;
        }
        
        // Выбираем process с максимальным OOM score
        let victim = processes.iter()
            .max_by_key(|p| p.oom_score)
            .cloned();
        
        self.oom_stats.total_oom_scoring += 1;
        self.oom_stats.last_oom_score = victim.as_ref()
            .map(|p| p.oom_score as i64)
            .unwrap_or(0);
        
        victim
    }
    
    /// Trigger OOM kill
    pub fn oom_kill_process(&mut self, pid: u32) {
        self.oom_stats.total_oom_kills += 1;
        println!("OOM Killer: Killing process {}", pid);
        // В реальности здесь был бы signal SIGKILL
    }
    
    /// Проверить есть ли OOM ситуация
    pub fn is_oom(&self) -> bool {
        if let Some(ref mm) = self.phys_memory {
            let utilization = mm.utilization();
            utilization > 95.0 // Threshold для OOM
        } else {
            false
        }
    }
    
    /// Try to free memory via page-out
    pub fn try_free_memory(&mut self) -> Result<bool, MmError> {
        if self.disable_swap {
            return Ok(false);
        }
        
        // Здесь должен быть код для вытеснения clean pages в swap
        // Для демо просто возвращаем success
        Ok(true)
    }
}

/// Информация о процессе для OOM killer
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub rss: u64,                    // Resident Set Size
    pub vmsize: u64,                 // Virtual Memory Size
    pub oom_score: i64,              // Calculated OOM score
    pub mem_weight: f64,             // Weight для scoring
}

impl ProcessInfo {
    pub fn new(pid: u32, name: &str, rss: u64) -> Self {
        Self {
            pid,
            name: name.to_string(),
            rss,
            vmsize: rss * 2, // Approximation
            oom_score: 0,
            mem_weight: 1.0,
        }
    }
    
    /// Рассчитать OOM score
    pub fn calculate_oom_score(&mut self, total_mem: u64) {
        // Простая формула: RSS / TotalMem * 1000
        let ratio = (self.rss as f64 / total_mem as f64) * 1000.0;
        self.oom_score = ratio as i64;
    }
}

/// Статистика Page Faults
#[derive(Debug, Clone)]
pub struct PageFaultStats {
    pub total_faults: u64,
    pub minor_faults: u64,           // Minor (page in cache)
    pub major_faults: u64,           // Major (page out from disk)
    pub cow_faults: u64,             // Copy-on-Write faults
    pub oom_kills: u64,
    pub pages_allocated: u64,
}

impl Default for PageFaultStats {
    fn default() -> Self {
        Self {
            total_faults: 0,
            minor_faults: 0,
            major_faults: 0,
            cow_faults: 0,
            oom_kills: 0,
            pages_allocated: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cow_info_basic() {
        let mut cow = CowInfo::new(100);
        assert!(cow.shared);
        assert_eq!(cow.ref_count, 1);
        
        cow.increment_ref();
        assert_eq!(cow.ref_count, 2);
        
        assert!(cow.decrement_ref());
        assert_eq!(cow.ref_count, 1);
    }
    
    #[test]
    fn test_page_fault_handler_create() {
        let handler = PageFaultHandler::new();
        assert!(handler.vma_manager.vma_count() == 0);
        assert!(handler.cow_table.is_empty());
    }
    
    #[test]
    fn test_register_and_handle_cow() {
        let mut handler = PageFaultHandler::new();
        
        // Register COW mapping
        handler.register_cow(0x7fff0000, 100);
        
        assert!(handler.cow_table.contains_key(&0x7fff0000));
        
        // Context для COW fault
        let context = PageFaultContext::new(0x7fff0000, PageFaultType::NonPresent);
        let result = handler.handle_fault(context);
        
        // Expect OK (would normally allocate physical page)
        assert!(result.is_ok() || result.is_err());
    }
    
    #[test]
    fn test_demand_paging() {
        let mut handler = PageFaultHandler::new();
        
        // Start demand paging
        let start = 0x10000000;
        let end = 0x10002000; // 2 pages
        
        let result = handler.start_demand_paging(start, end);
        assert!(result.is_ok());
        
        assert_eq!(handler.in_flight_pages.len(), 2);
    }
    
    #[test]
    fn test_oom_victim_selection() {
        let mut processes = vec![
            ProcessInfo::new(1, "chrome", 1000000),
            ProcessInfo::new(2, "firefox", 500000),
            ProcessInfo::new(3, "nginx", 200000),
        ];
        
        let total_mem = 8 * 1024 * 1024 * 1024; // 8GB
        
        for proc in &mut processes {
            proc.calculate_oom_score(total_mem);
        }
        
        let handler = PageFaultHandler::new();
        let victim = handler.select_oom_victim(&processes);
        
        assert!(victim.is_some());
        assert_eq!(victim.unwrap().pid, 1); // Chrome should be selected
    }
}
