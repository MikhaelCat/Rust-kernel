//! Statistics module for kernel subsystems

/// Combined statistics for all kernel systems
#[derive(Debug, Clone, Default)]
pub struct KernelStats {
    pub boot_stats: BootStats,
    pub sched_stats: SchedStats,
    pub mm_stats: MmStats,
    pub ipc_stats: IpcStats,
    pub power_stats: PowerStats,
    pub crypto_stats: CryptoStats,
}
