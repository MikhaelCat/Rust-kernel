#[derive(Debug, Default)]
pub struct SignalFd {
    pending: u64,
}
impl SignalFd {
    pub fn raise(&mut self) {
        self.pending += 1;
    }
    pub fn pending(&self) -> u64 {
        self.pending
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signal_pending() {
        let mut s = SignalFd::default();
        s.raise();
        assert_eq!(s.pending(), 1);
    }
}
