//! Power management statistics

#[derive(Debug, Clone, Default)]
pub struct PowerStats {
    pub energy_consumed: u64,
    pub thermal_throttling_events: u64,
}
