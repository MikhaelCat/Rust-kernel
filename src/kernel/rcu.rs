#[derive(Debug, Default)]
pub struct RcuState {
    grace_periods: u64,
}
impl RcuState {
    pub fn quiescent_cycle(&mut self) {
        self.grace_periods += 1;
    }
    pub fn gps(&self) -> u64 {
        self.grace_periods
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rcu_gp() {
        let mut r = RcuState::default();
        r.quiescent_cycle();
        assert_eq!(r.gps(), 1);
    }
}
