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

    #[test]
    fn integration_alloc_map_vma() {
        let mut mm = PhysicalMemoryManager::new(4096 * 4, 4096).expect("init failed");
        let page = mm.allocate_page().expect("alloc failed");

        let mut pt = PageTable::default();
        pt.map(0x4000, page.index * 4096);

        let mut vmas = VmaMap::default();
        vmas.insert(0x4000, 0x5000).expect("vma failed");

        assert_eq!(pt.translate(0x4000), Some(0));
        assert_eq!(vmas.count(), 1);
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

pub mod manager;
pub use manager::MemoryManager;

pub mod debug;
pub mod stats;

pub mod sanitizer;

pub mod components;

pub mod protection;

pub mod cow;
pub mod fault;

pub mod semantics;
