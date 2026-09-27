//! Virtualization Support for Linux Kernel on Rust - KVM, QEMU

pub mod checkpoint;
pub mod manager;
pub mod migration;
pub mod vm;

pub use manager::VirtManager;
pub use vm::Vm;

pub mod recovery;
pub mod snapshot;
pub mod lifecycle;
pub mod manager2;
pub mod stress;
pub mod block;
pub mod device;
pub mod memory;
pub mod network;

#[derive(Debug, Clone)]
pub struct VirtStats {
    pub active_vms: u32,
    pub migrations_performed: u32,
    pub snapshots_created: u32,
    pub checkpoint_count: u32,
}

impl Default for VirtStats {
    fn default() -> Self {
        Self {
            active_vms: 0,
            migrations_performed: 0,
            snapshots_created: 0,
            checkpoint_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QemuKvmStats {
    pub vcpu_cycles_executed: u64,
    pub kvm_io_events: u64,
    pub virtio_devices: u32,
}

#[derive(Debug, Clone)]
pub struct MigrationStats {
    pub migration_pages_sent: u64,
    pub migration_time_ms: u64,
    pub migration_downtime_us: u64,
}
