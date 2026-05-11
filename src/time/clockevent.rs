#[derive(Debug, Default)]
pub struct ClockEvent {
    fired: u64,
}
impl ClockEvent {
    pub fn fire(&mut self) {
        self.fired += 1;
    }
    pub fn fired(&self) -> u64 {
        self.fired
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fire_event() {
        let mut c = ClockEvent::default();
        c.fire();
        assert_eq!(c.fired(), 1);
    }
}
