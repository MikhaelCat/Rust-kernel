#[derive(Debug, Default)]
pub struct BootWatchdog {
    kicks: u64,
}
impl BootWatchdog {
    pub fn kick(&mut self) {
        self.kicks += 1;
    }
    pub fn kicks(&self) -> u64 {
        self.kicks
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kick_once() {
        let mut w = BootWatchdog::default();
        w.kick();
        assert_eq!(w.kicks(), 1);
    }
}
