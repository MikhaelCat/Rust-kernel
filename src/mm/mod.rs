//! Memory Management for Linux Kernel on Rust

pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;
pub mod slab;

pub mod error;
pub mod pager;
pub mod phys;
pub mod vm;

pub use error::MmError;
pub use pager::PageTable;
pub use phys::{PageFrame, PhysicalMemoryManager};
pub use vm::{VmArea, VmaMap};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocator_lifecycle() {
        let mut mm = PhysicalMemoryManager::new(4096 * 2, 4096).expect("init failed");
        let page = mm.allocate_page().expect("alloc failed");
        assert_eq!(mm.used_pages(), 1);
        mm.free_page(page).expect("free failed");
        assert_eq!(mm.used_pages(), 0);
    }

    #[test]
    fn page_table_map_unmap() {
        let mut pt = PageTable::default();
        pt.map(0x1000, 0x2000);
        assert_eq!(pt.translate(0x1000), Some(0x2000));
        pt.unmap(0x1000).expect("unmap failed");
        assert_eq!(pt.translate(0x1000), None);
    }

    #[test]
    fn vma_insert_validation() {
        let mut vm = VmaMap::default();
        assert!(vm.insert(0x1000, 0x2000).is_ok());
        assert_eq!(vm.insert(0x3000, 0x3000), Err(MmError::InvalidRange));
        assert_eq!(vm.count(), 1);
    }
}

pub mod cma;
pub mod compaction;
pub mod hugepage;
pub mod kasan;
pub mod ksm;
pub mod memcg;
pub mod oom;
pub mod page_owner;
pub mod reclaim;
pub mod swap;
pub mod vmalloc;
pub mod zones;

// Slab allocator and virtual memory
pub mod kmalloc;
pub mod fault;

pub mod manager;
pub use manager::MemoryManager;

// Debug and stats
pub mod debug;
pub mod stats;

// Advanced features
pub mod sanitizer;
pub mod components;
pub mod protection;
pub mod cow;
pub mod semantics;

#[derive(Debug, Clone)]
pub struct MmStats {
    pub total_pages: u64,
    pub free_pages: u64,
    pub used_pages: u64,
    pub cached_pages: u64,
}

impl Default for MmStats {
    fn default() -> Self {
        Self {
            total_pages: 0,
            free_pages: 0,
            used_pages: 0,
            cached_pages: 0,
        }
    }
}

impl MmStats {
    pub fn page_utilization(&self) -> f64 {
        if self.total_pages == 0 { return 0.0; }
        (self.used_pages as f64 / self.total_pages as f64) * 100.0
    }
}

#[derive(Debug, Clone)]
pub struct PageCacheStats {
    pub cached_pages: u64,
    pub dirty_pages: u64,
    pub writeback_pages: u64,
    pub pgscan_direct: u64,
    pub pgscan_kswapd: u64,
}

#[derive(Debug, Clone)]
pub struct SwapStats {
    pub total_swap: u64,
    pub free_swap: u64,
    pub used_swap: u64,
    pub pages_swapped_in: u64,
    pub pages_swapped_out: u64,
}
