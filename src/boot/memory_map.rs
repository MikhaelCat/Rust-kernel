#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
}

#[derive(Debug, Default)]
pub struct MemoryMap {
    regions: Vec<MemoryRegion>,
}

impl MemoryMap {
    pub fn add_region(&mut self, start: u64, size: u64) {
        self.regions.push(MemoryRegion { start, size });
    }

    pub fn total_size(&self) -> u64 {
        self.regions.iter().map(|r| r.size).sum()
    }

    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    pub fn is_valid(&self) -> bool {
        if self.regions.is_empty() {
            return false;
        }
        self.regions.iter().all(|r| r.size > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_region_sizes() {
        let mut m = MemoryMap::default();
        m.add_region(0, 1024);
        m.add_region(1024, 2048);
        assert_eq!(m.total_size(), 3072);
        assert_eq!(m.region_count(), 2);
        assert!(m.is_valid());
    }

    #[test]
    fn invalid_when_empty_or_zero_sized() {
        let m = MemoryMap::default();
        assert!(!m.is_valid());

        let mut m2 = MemoryMap::default();
        m2.add_region(0, 0);
        assert!(!m2.is_valid());
    }
}
