//! Статистика всех компонентов Linux-ядра на Rust

use crate::mm::MmStats;
use crate::fs::FsStats;
use crate::sched::CfsStats;
use crate::net::NetStats;
use crate::security::SecurityStats;
use crate::time::TimeStats;
use crate::block::BlockStats;
use crate::drivers::DriverStats;
use crate::ipc::IpcStats;
use crate::virt::VirtStats;
use crate::boot::BootStats;
use crate::power::PowerStats;
use crate::crypto::CryptoStats;
use crate::io_uring::IoUringStats;

/// Общая статистика системы ядра Linux
#[derive(Debug, Clone)]
pub struct KernelStats {
    pub boot: BootStats,
    pub memory: MmStats,
    pub filesystem: FsStats,
    pub scheduler: CfsStats,
    pub network: NetStats,
    pub security: SecurityStats,
    pub time: TimeStats,
    pub block: BlockStats,
    pub drivers: DriverStats,
    pub ipc: IpcStats,
    pub virtualization: VirtStats,
    pub power: PowerStats,
    pub crypto: CryptoStats,
    pub io_uring: IoUringStats,
}

impl Default for KernelStats {
    fn default() -> Self {
        Self {
            boot: BootStats::default(),
            memory: MmStats::default(),
            filesystem: FsStats::default(),
            scheduler: CfsStats::default(),
            network: NetStats::default(),
            security: SecurityStats::default(),
            time: TimeStats::default(),
            block: BlockStats::default(),
            drivers: DriverStats::default(),
            ipc: IpcStats::default(),
            virtualization: VirtStats::default(),
            power: PowerStats::default(),
            crypto: CryptoStats::default(),
            io_uring: IoUringStats::default(),
        }
    }
}

impl KernelStats {
    /// Получить общую утилизацию памяти (проценты)
    pub fn memory_usage(&self) -> f64 {
        self.memory.page_utilization()
    }

    /// Получить утилизацию файловой системы (проценты)
    pub fn filesystem_usage(&self) -> f64 {
        self.filesystem.space_utilization()
    }

    /// Получить denial rate безопасности (проценты)
    pub fn security_denial_rate(&self) -> f64 {
        self.security.denial_rate()
    }

    /// Суммарное количество переданных байт в сети
    pub fn total_network_bytes(&self) -> u64 {
        self.network.bytes_transferred
    }

    /// Суммарная скорость ввода-вывода (байт/сек)
    pub fn io_throughput(&self) -> u64 {
        self.block.throughput_bytes() + self.io_uring.completions as u64
    }

    /// Общее количество активных процессов
    pub fn active_processes(&self) -> u32 {
        self.scheduler.nr_runs as u32
    }

    /// Активные виртуальные машины
    pub fn active_vms(&self) -> u32 {
        self.virtualization.active_vms
    }

    /// Генерировать сводку статистики
    pub fn summary(&self) -> String {
        format!(
            "Linux Kernel Stats on Rust\n\
             Memory Usage: {:.1}%\n\
             Filesystem Usage: {:.1}%\n\
             Active Processes: {}\n\
             Network Bytes Transferred: {}\n\
             IO Throughput: {} bytes/sec\n\
             Active VMs: {}\n",
            self.memory_usage(),
            self.filesystem_usage(),
            self.active_processes(),
            self.total_network_bytes(),
            self.io_throughput(),
            self.active_vms()
        )
    }
}
