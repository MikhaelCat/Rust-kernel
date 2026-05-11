#[derive(Debug, Default)]
pub struct MemCg {
    pub used: usize,
    pub limit: usize,
}
impl MemCg {
    pub fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }
    pub fn charge(&mut self, bytes: usize) -> bool {
        if self.used + bytes <= self.limit {
            self.used += bytes;
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn memcg_charge() {
        let mut m = MemCg::default();
        m.set_limit(1024);
        assert!(m.charge(512));
        assert!(!m.charge(600));
    }
}
