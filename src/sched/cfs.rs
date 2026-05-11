#[derive(Debug, Default)]
pub struct CfsStats {
    pub vruntime: u64,
}
impl CfsStats {
    pub fn tick(&mut self) {
        self.vruntime += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cfs_tick() {
        let mut s = CfsStats::default();
        s.tick();
        assert_eq!(s.vruntime, 1);
    }
}
