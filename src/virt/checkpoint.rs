//! Migration and Checkpoint for Virtual Machines

use crate::vm::Vm;

#[derive(Debug, Clone)]
pub struct Snapshot {
    pub vm_id: u32,
    pub snapshot_id: u32,
    pub memory_dump: Vec<u8>,
    pub timestamp: u64,
}

impl Snapshot {
    pub fn new(vm_id: u32, mem_data: &[u8]) -> Self {
        Self {
            vm_id,
            snapshot_id: 0,
            memory_dump: mem_data.to_vec(),
            timestamp: 0,
        }
    }
}

#[derive(Debug)]
pub struct MigrationState {
    pub active: bool,
    pub pages_sent: u64,
    pub downtime_us: u64,
}

impl Default for MigrationState {
    fn default() -> Self {
        Self {
            active: false,
            pages_sent: 0,
            downtime_us: 0,
        }
    }
}

#[derive(Debug)]
pub struct CheckpointManager {
    pub snapshots: Vec<Snapshot>,
    pub migration: MigrationState,
    pub checkpoint_count: u32,
}

impl Default for CheckpointManager {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckpointManager {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            migration: MigrationState::default(),
            checkpoint_count: 0,
        }
    }

    pub fn create_snapshot(&mut self, vm: &Vm) -> u32 {
        self.snapshots.push(Snapshot::new(vm.vm_id, &[]));
        self.checkpoint_count += 1;
        self.checkpoint_count
    }

    pub fn start_migration(&mut self) {
        self.migration.active = true;
    }

    pub fn stop_migration(&mut self) {
        self.migration.active = false;
    }

    pub fn get_snapshot(&self, index: usize) -> Option<&Snapshot> {
        self.snapshots.get(index)
    }

    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}
