#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmMemory {
    pub size_mb: u32,
    pub balloon_mb: u32,
}

impl VmMemory {
    pub fn new(size_mb: u32) -> Self {
        Self {
            size_mb,
            balloon_mb: 0,
        }
    }

    pub fn balloon(&mut self, mb: u32) {
        self.balloon_mb = mb.min(self.size_mb);
    }

    pub fn effective_mb(&self) -> u32 {
        self.size_mb.saturating_sub(self.balloon_mb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ballooning() {
        let mut m = VmMemory::new(1024);
        m.balloon(128);
        assert_eq!(m.effective_mb(), 896);
    }
}
