#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    pub hz: u64,
}
impl Clock {
    pub fn new(hz: u64) -> Self {
        Self { hz }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clock_new() {
        assert_eq!(Clock::new(1000).hz, 1000);
    }
}
