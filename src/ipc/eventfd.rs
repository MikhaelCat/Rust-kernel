#[derive(Debug, Default)]
pub struct EventFd {
    cnt: u64,
}
impl EventFd {
    pub fn signal(&mut self) {
        self.cnt += 1;
    }
    pub fn read(&self) -> u64 {
        self.cnt
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signal_event() {
        let mut e = EventFd::default();
        e.signal();
        assert_eq!(e.read(), 1);
    }
}
