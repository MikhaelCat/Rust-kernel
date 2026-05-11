#[derive(Debug, Default)]
pub struct Tlb {
    flushes: u64,
}
impl Tlb {
    pub fn flush_all(&mut self) {
        self.flushes += 1;
    }
    pub fn flushes(&self) -> u64 {
        self.flushes
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tlb_flush() {
        let mut t = Tlb::default();
        t.flush_all();
        assert_eq!(t.flushes(), 1);
    }
}
