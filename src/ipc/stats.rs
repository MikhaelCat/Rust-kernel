//! IPC subsystem statistics

#[derive(Debug, Clone, Default)]
pub struct IpcStats {
    pub shared_memory_segments: u64,
    pub semaphores: u64,
    pub message_queues: u64,
}
