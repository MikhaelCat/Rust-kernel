//! Stub types for missing statistics

/// Boot subsystem statistics
#[derive(Debug, Clone, Default)]
pub struct BootStats {
    pub boot_time_ms: u64,
}

/// Scheduler statistics  
#[derive(Debug, Clone, Default)]
pub struct SchedStats {
    pub tasks_created: u64,
}

/// Memory management statistics
#[derive(Debug, Clone, Default)]
pub struct MmStats {
    pub pages_allocated: u64,
}

/// IPC subsystem statistics
#[derive(Debug, Clone, Default)]
pub struct IpcStats {
    pub messages_sent: u64,
}

/// Power management statistics
#[derive(Debug, Clone, Default)]
pub struct PowerStats {
    pub power_events: u64,
}

/// Cryptography statistics
#[derive(Debug, Clone, Default)]
pub struct CryptoStats {
    pub operations_completed: u64,
}

/// Driver statistics
#[derive(Debug, Clone, Default)]
pub struct DriverStats {
    pub drivers_loaded: u64,
}
