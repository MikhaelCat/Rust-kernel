//! Boot subsystem statistics

#[derive(Debug, Clone, Default)]
pub struct BootStats {
    pub init_time_ns: u64,
    pub modules_loaded: u64,
}
