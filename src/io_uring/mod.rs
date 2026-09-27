//! Async I/O - io_uring Implementation

pub mod cqe;
pub mod engine;
pub mod op;
pub mod queue;
pub mod sqe;

pub use engine::IoUringEngine;
pub use op::IoOp;
pub use queue::{CompletionQueue, SubmissionQueue};

pub mod stress;

#[derive(Debug, Clone)]
pub struct IoUringStats {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub submissions: u64,
    pub completions: u64,
}

impl Default for IoUringStats {
    fn default() -> Self {
        Self {
            sq_entries: 0,
            cq_entries: 0,
            submissions: 0,
            completions: 0,
        }
    }
}
