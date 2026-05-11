#[derive(Debug, Default)]
pub struct TimerWheel {
    tick: u64,
}
impl TimerWheel {
    pub fn advance(&mut self, by: u64) {
        self.tick += by;
    }
    pub fn now(&self) -> u64 {
        self.tick
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tick_advances() {
        let mut t = TimerWheel::default();
        t.advance(5);
        assert_eq!(t.now(), 5);
    }
}
