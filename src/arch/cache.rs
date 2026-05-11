#[derive(Debug, Default)]
pub struct CpuCache {
    invalidates: u64,
}
impl CpuCache {
    pub fn invalidate_all(&mut self) {
        self.invalidates += 1;
    }
    pub fn invalidates(&self) -> u64 {
        self.invalidates
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn invalidate() {
        let mut c = CpuCache::default();
        c.invalidate_all();
        assert_eq!(c.invalidates(), 1);
    }
}
