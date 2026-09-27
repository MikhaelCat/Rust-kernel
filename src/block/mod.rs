//! Block subsystem statistics

#[derive(Debug, Clone, Default)]
pub struct BlockStats {
    pub io_count: u64,
    pub bytes_processed: u64,
}
