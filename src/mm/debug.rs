use super::manager::MemoryManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmSnapshot {
    pub used_pages: usize,
}

pub fn snapshot(mm: &MemoryManager) -> MmSnapshot {
    MmSnapshot {
        used_pages: mm.used_pages(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_reports_usage() {
        let mut mm = MemoryManager::new(4096 * 2, 4096).expect("init failed");
        let _ = mm.map_new_page(0x1000).expect("map failed");
        let s = snapshot(&mm);
        assert_eq!(s.used_pages, 1);
    }
}
