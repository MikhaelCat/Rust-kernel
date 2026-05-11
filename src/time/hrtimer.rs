#[derive(Debug, Default)]
pub struct HrTimer {
    ns: u64,
}
impl HrTimer {
    pub fn advance_ns(&mut self, delta: u64) {
        self.ns += delta;
    }
    pub fn now_ns(&self) -> u64 {
        self.ns
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hrtimer_advance() {
        let mut t = HrTimer::default();
        t.advance_ns(10);
        assert_eq!(t.now_ns(), 10);
    }
}
