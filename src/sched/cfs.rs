//! CFS (Completely Fair Scheduler) stats

#[derive(Debug, Clone, Default)]
pub struct CfsStats {
    pub runnable_weight: u64,
    pub weighted_exec_time: u64,
}
