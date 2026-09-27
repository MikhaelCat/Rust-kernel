//! Linux Kernel Scheduler Module - All Algorithms

pub mod cfs;
pub mod deadline;
pub mod error;
pub mod fair;
pub mod loadavg;
pub mod manager;
pub mod preempt;
pub mod rt;
pub mod topology;
pub mod types;

pub use error::SchedError;
pub use manager::SchedulerManager;

pub mod system;
pub use system::{SchedSnapshot, SchedSystem};

#[derive(Debug, Clone)]
pub struct SchedStats {
    pub context_switches: u64,
    pub run_queue_depth: u32,
    pub load_avg_1min: f64,
    pub load_avg_5min: f64,
    pub load_avg_15min: f64,
}

impl Default for SchedStats {
    fn default() -> Self {
        Self {
            context_switches: 0,
            run_queue_depth: 0,
            load_avg_1min: 0.0,
            load_avg_5min: 0.0,
            load_avg_15min: 0.0,
        }
    }
}

impl SchedStats {
    pub fn avg_load(&self) -> f64 {
        (self.load_avg_1min + self.load_avg_5min + self.load_avg_15min) / 3.0
    }
}

#[derive(Debug, Clone)]
pub struct RtPolicyStats {
    pub realtime_tasks: u32,
    pub batch_tasks: u32,
    pub idle_tasks: u32,
}
