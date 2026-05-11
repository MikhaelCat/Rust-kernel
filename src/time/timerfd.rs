#[derive(Debug, Default)]
pub struct TimerFd {
    expirations: u64,
}
impl TimerFd {
    pub fn tick(&mut self) {
        self.expirations += 1;
    }
    pub fn expirations(&self) -> u64 {
        self.expirations
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timerfd_tick() {
        let mut t = TimerFd::default();
        t.tick();
        assert_eq!(t.expirations(), 1);
    }
}
