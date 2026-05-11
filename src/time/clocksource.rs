#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClockSource {
    pub name: &'static str,
    pub freq_hz: u64,
}
impl ClockSource {
    pub fn monotonic() -> Self {
        Self {
            name: "monotonic",
            freq_hz: 1_000_000_000,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn clocksource_default() {
        assert_eq!(ClockSource::monotonic().name, "monotonic");
    }
}
