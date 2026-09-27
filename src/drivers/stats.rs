//! Driver statistics

#[derive(Debug, Clone, Default)]
pub struct DriverStats {
    pub loaded_drivers: u64,
    pub active_devices: u64,
}
