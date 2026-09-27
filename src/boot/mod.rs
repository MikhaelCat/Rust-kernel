//! Boot Process Implementation for Linux Kernel on Rust
//! Реализация процесса загрузки ядра Linux на Rust

use std::collections::HashMap;

// ============================================================================
// BOOT INTERFACE
// ============================================================================

#[derive(Debug)]
pub struct BootInfo {
    pub magic: u32,                // BOOT_MAGIC = 0x19870523
    pub mem_upper_bound: u32,
    pub boot_device: u32,
    pub cmdline: String,
    pub modules: Vec<BootModule>,
    pub mmap_entries: u32,
    pub acpi_rsdp_addr: usize,
    pub total_memory: usize,
    pub num_cpus: u32,
    pub is_64bit: bool,
}

const BOOT_MAGIC: u32 = 0x19870523;

#[derive(Debug)]
pub struct BootModule {
    pub start_addr: usize,
    pub end_addr: usize,
    pub name: String,
    pub cmdline: String,
    pub type_: ModuleType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    Unknown,
    KernelModules,
    Initramfs,
    DeviceTree,
    Reserved,
}

impl Default for BootInfo {
    fn default() -> Self {
        Self {
            magic: 0,
            mem_upper_bound: 0,
            boot_device: 0,
            cmdline: String::new(),
            modules: Vec::new(),
            mmap_entries: 0,
            acpi_rsdp_addr: 0,
            total_memory: 0,
            num_cpus: 0,
            is_64bit: true,
        }
    }
}

// ============================================================================
// MEMORY MAP
// ============================================================================

#[derive(Debug)]
pub struct MemoryMap {
    regions: Vec<MemoryRegion>,
    entries_count: u32,
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            regions: Vec::new(),
            entries_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub phys_addr: usize,
    pub size: usize,
    pub region_type: RegionType,
    pub attributes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    Usable,
    Reserved,
    ACPI_NVS,
    BadMemory,
    BootLoader,
    LoaderData,
    KernelUsed,
}

impl MemoryMap {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add memory region
    pub fn add_region(&mut self, addr: usize, size: usize, rtype: RegionType) {
        self.regions.push(MemoryRegion {
            phys_addr: addr,
            size,
            region_type: rtype,
            attributes: 0,
        });
        
        self.entries_count += 1;
    }
    
    /// Get usable memory regions
    pub fn get_usable_regions(&self) -> Vec<&MemoryRegion> {
        self.regions.iter().filter(|r| r.region_type == RegionType::Usable).collect()
    }
    
    /// Get total usable memory
    pub fn total_usable_memory(&self) -> usize {
        self.get_usable_regions().iter()
            .map(|r| r.size)
            .sum()
    }
    
    /// Reserve memory region
    pub fn reserve_region(&mut self, addr: usize, size: usize) -> Result<(), MemoryError> {
        if self.find_region(addr, size).is_some() {
            return Ok(()); // Already reserved
        }
        
        self.add_region(addr, size, RegionType::Reserved);
        Ok(())
    }
    
    fn find_region(&self, addr: usize, size: usize) -> Option<&MemoryRegion> {
        self.regions.iter()
            .find(|r| {
                r.phys_addr <= addr && 
                (addr + size) <= (r.phys_addr + r.size)
            })
    }
}

// ============================================================================
// KERNEL PARAMETER PARSER
// ============================================================================

#[derive(Debug)]
pub struct KernelParams {
    pub console: Option<String>,
    pub root: Option<String>,
    pub init: Option<String>,
    pub debug: bool,
    pub nomap: bool,
    pub early_printk: bool,
    pub cpus: u32,
    pub quiet: bool,
    pub single: bool,
    pub noapic: bool,
    pub pci: String,
    pub loglevel: u32,
}

impl Default for KernelParams {
    fn default() -> Self {
        Self {
            console: None,
            root: None,
            init: None,
            debug: false,
            nomap: false,
            early_printk: false,
            cpus: 1,
            quiet: false,
            single: false,
            noapic: false,
            pci: String::new(),
            loglevel: 3,
        }
    }
}

impl KernelParams {
    pub fn parse(cmdline: &str) -> Self {
        let mut params = Self::default();
        
        for arg in cmdline.split_whitespace() {
            if let Some((key, value)) = arg.split_once('=') {
                match key {
                    "console" => params.console = Some(value.to_string()),
                    "root" => params.root = Some(value.to_string()),
                    "init" => params.init = Some(value.to_string()),
                    "cpus" => {
                        if let Ok(n) = value.parse() {
                            params.cpus = n;
                        }
                    },
                    "loglevel" => {
                        if let Ok(lvl) = value.parse() {
                            params.loglevel = lvl;
                        }
                    },
                    "pci" => params.pci = value.to_string(),
                    _ => {},
                }
            } else {
                match arg {
                    "debug" => params.debug = true,
                    "nomap" => params.nomap = true,
                    "quiet" => params.quiet = true,
                    "single" => params.single = true,
                    "noapic" => params.noapic = true,
                    "earlyprintk" => params.early_printk = true,
                    _ => {},
                }
            }
        }
        
        params
    }
    
    pub fn has(&self, param: &str) -> bool {
        match param {
            "debug" => self.debug,
            "nomap" => self.nomap,
            "quiet" => self.quiet,
            "single" => self.single,
            "noapic" => self.noapic,
            "earlyprintk" => self.early_printk,
            _ => false,
        }
    }
}

// ============================================================================
// INITIALIZATION LOG
// ============================================================================

pub struct BootLogger {
    messages: Vec<String>,
    initialized: bool,
}

impl Default for BootLogger {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            initialized: false,
        }
    }
}

impl BootLogger {
    pub fn info(&mut self, msg: &str) {
        self.messages.push(format!("[INFO] {}", msg));
    }
    
    pub fn warning(&mut self, msg: &str) {
        self.messages.push(format!("[WARN] {}", msg));
    }
    
    pub fn error(&mut self, msg: &str) {
        self.messages.push(format!("[ERROR] {}", msg));
    }
    
    pub fn print_all(&self) {
        println!("=== BOOT LOG ===");
        for msg in &self.messages {
            println!("{}", msg);
        }
        println!("================");
    }
    
    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryError {
    AddressNotFound,
    InvalidAddress,
    RegionConflict,
    OutOfMemory,
}

impl std::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryError::AddressNotFound => write!(f, "Address not found"),
            MemoryError::InvalidAddress => write!(f, "Invalid address"),
            MemoryError::RegionConflict => write!(f, "Region conflict"),
            MemoryError::OutOfMemory => write!(f, "Out of memory"),
        }
    }
}

impl std::error::Error for MemoryError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_boot_info_creation() {
        let info = BootInfo::default();
        assert_eq!(info.magic, 0);
    }
    
    #[test]
    fn test_memory_map_add_regions() {
        let mut map = MemoryMap::new();
        
        map.add_region(0x00000000, 0x10000, RegionType::Usable);
        map.add_region(0x100000, 0x10000, RegionType::Reserved);
        
        assert_eq!(map.regions.len(), 2);
        assert!(map.entries_count > 0);
    }
    
    #[test]
    fn test_kernel_params_parsing() {
        let cmdline = "root=/dev/sda1 quiet debug loglevel=5";
        let params = KernelParams::parse(cmdline);
        
        assert_eq!(params.root, Some("/dev/sda1".to_string()));
        assert!(params.quiet);
        assert!(params.debug);
        assert_eq!(params.loglevel, 5);
    }
    
    #[test]
    fn test_boot_logger() {
        let mut logger = BootLogger::default();
        
        logger.info("Kernel starting");
        logger.warning("Low memory");
        logger.error("Fatal error");
        
        assert_eq!(logger.messages.len(), 3);
    }
    
    #[test]
    fn test_memory_reservation() {
        let mut map = MemoryMap::new();
        
        map.reserve_region(0x10000, 0x1000).unwrap();
        
        let usable = map.get_usable_regions();
        assert!(!usable.is_empty());
    }
}
