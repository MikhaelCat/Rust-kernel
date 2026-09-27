//! System Call Interface for Linux Kernel on Rust

pub mod clone;
pub mod errno;
pub mod execve;
pub mod fcntl;
pub mod fd_ops;
pub mod fs;
pub mod fs_ops;
pub mod harness;
pub mod harness_edge;
pub mod memory;
pub mod memory_flags;
pub mod net;
pub mod net_ext;
pub mod poll;
pub mod process;
pub mod process_semantics;
pub mod process_table;
pub mod signal;
pub mod sync;
pub mod syscall;
pub mod virt_hooks;
pub mod wait;
pub mod waitpid;

pub use harness::{AbiComplianceReport, run_abi_compliance};
pub use harness_edge::{AbiEdgeReport, run_abi_edge_checks};
pub use syscall::{AbiKernel, Sysno};

#[derive(Debug, Clone)]
pub struct SyscallStats {
    pub total_syscalls: u64,
    pub syscalls_by_type: Vec<u64>,
    pub syscall_errors: u32,
    pub avg_latency_ns: u64,
}

impl Default for SyscallStats {
    fn default() -> Self {
        Self {
            total_syscalls: 0,
            syscalls_by_type: vec![0; 512],
            syscall_errors: 0,
            avg_latency_ns: 0,
        }
    }
}
