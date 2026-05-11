use super::cow::CowTracker;
use super::fault::{FaultAction, PageFault, handle_fault};
use super::reclaim::Reclaimer;
use super::{MmError, PageFrame, PageTable, PhysicalMemoryManager, VmaMap};

#[derive(Debug)]
pub struct MemoryManager {
    phys: PhysicalMemoryManager,
    pt: PageTable,
    vmas: VmaMap,
    cow: CowTracker,
    reclaim: Reclaimer,
    faults_handled: u64,
    kills: u64,
}

impl MemoryManager {
    pub fn new(total_memory_bytes: usize, page_size: usize) -> Result<Self, MmError> {
        Ok(Self {
            phys: PhysicalMemoryManager::new(total_memory_bytes, page_size)?,
            pt: PageTable::default(),
            vmas: VmaMap::default(),
            cow: CowTracker::default(),
            reclaim: Reclaimer::default(),
            faults_handled: 0,
            kills: 0,
        })
    }

    pub fn map_new_page(&mut self, va: usize) -> Result<PageFrame, MmError> {
        let page = self.phys.allocate_page()?;
        let pa = page.index * self.phys.page_size();
        self.pt.map(va, pa);
        Ok(page)
    }

    pub fn unmap_and_free_page(&mut self, va: usize, page: PageFrame) -> Result<(), MmError> {
        self.pt.unmap(va)?;
        self.phys.free_page(page)
    }

    pub fn map_vma(&mut self, start: usize, end: usize) -> Result<(), MmError> {
        self.vmas.insert(start, end)
    }

    pub fn translate(&self, va: usize) -> Option<usize> {
        self.pt.translate(va)
    }

    pub fn used_pages(&self) -> usize {
        self.phys.used_pages()
    }

    pub fn handle_page_fault(
        &mut self,
        va: usize,
        fault: PageFault,
    ) -> Result<FaultAction, MmError> {
        match handle_fault(fault) {
            FaultAction::MapPage => {
                let _ = self.map_new_page(va)?;
                self.faults_handled += 1;
                Ok(FaultAction::MapPage)
            }
            FaultAction::KillTask => {
                self.kills += 1;
                Ok(FaultAction::KillTask)
            }
        }
    }

    pub fn write_shared_page(&mut self, va: usize) -> Result<(), MmError> {
        if self.translate(va).is_none() {
            return Err(MmError::UnmappedAddress);
        }
        self.cow.on_write_shared_page();
        Ok(())
    }

    pub fn reclaim_if_needed(&mut self, high_watermark_pages: usize) -> usize {
        if self.used_pages() <= high_watermark_pages {
            return 0;
        }
        self.reclaim.scan();
        self.used_pages().saturating_sub(high_watermark_pages)
    }

    pub fn mm_counters(&self) -> (u64, u64, u64, u64) {
        (
            self.faults_handled,
            self.kills,
            self.cow.copies(),
            self.reclaim.scanned(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_and_unmap_flow() {
        let mut mm = MemoryManager::new(4096 * 8, 4096).expect("init failed");
        let page = mm.map_new_page(0x1000).expect("map failed");
        assert_eq!(mm.translate(0x1000), Some(0));
        assert_eq!(mm.used_pages(), 1);

        mm.unmap_and_free_page(0x1000, page)
            .expect("unmap/free failed");
        assert_eq!(mm.translate(0x1000), None);
        assert_eq!(mm.used_pages(), 0);
    }

    #[test]
    fn vma_flow() {
        let mut mm = MemoryManager::new(4096 * 8, 4096).expect("init failed");
        assert!(mm.map_vma(0x4000, 0x5000).is_ok());
        assert_eq!(mm.map_vma(0x6000, 0x6000), Err(MmError::InvalidRange));
    }

    #[test]
    fn page_fault_map_and_protection_kill() {
        let mut mm = MemoryManager::new(4096 * 8, 4096).expect("init failed");
        assert_eq!(
            mm.handle_page_fault(0x2000, PageFault::NotPresent)
                .expect("fault"),
            FaultAction::MapPage
        );
        assert_eq!(mm.translate(0x2000), Some(0));

        assert_eq!(
            mm.handle_page_fault(0x2000, PageFault::Protection)
                .expect("fault"),
            FaultAction::KillTask
        );

        let (faults, kills, _, _) = mm.mm_counters();
        assert_eq!(faults, 1);
        assert_eq!(kills, 1);
    }

    #[test]
    fn cow_and_reclaim_counters() {
        let mut mm = MemoryManager::new(4096 * 8, 4096).expect("init failed");
        let _p0 = mm.map_new_page(0x1000).expect("map0");
        let _p1 = mm.map_new_page(0x2000).expect("map1");
        mm.write_shared_page(0x1000).expect("cow");
        let reclaimed = mm.reclaim_if_needed(1);
        assert!(reclaimed >= 1);
        let (_, _, cow, scanned) = mm.mm_counters();
        assert_eq!(cow, 1);
        assert_eq!(scanned, 1);
    }
}
