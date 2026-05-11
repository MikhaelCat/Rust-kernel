#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MmStats {
    pub alloc_ops: u64,
    pub free_ops: u64,
}

impl MmStats {
    pub fn on_alloc(&mut self) {
        self.alloc_ops += 1;
    }

    pub fn on_free(&mut self) {
        self.free_ops += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_increment() {
        let mut s = MmStats::default();
        s.on_alloc();
        s.on_free();
        assert_eq!(s.alloc_ops, 1);
        assert_eq!(s.free_ops, 1);
    }
}
