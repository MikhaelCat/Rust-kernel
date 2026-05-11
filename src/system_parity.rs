use crate::abi::{run_abi_compliance, run_abi_edge_checks};
use crate::compat::{LinuxSyscallClass, linux_syscall_class, linux_syscall_name};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParityReport {
    pub abi_base_ok: bool,
    pub abi_edge_ok: bool,
    pub known_syscalls: usize,
    pub network_class_ok: bool,
}

impl ParityReport {
    pub fn all_ok(&self) -> bool {
        self.abi_base_ok && self.abi_edge_ok && self.known_syscalls >= 10 && self.network_class_ok
    }
}

pub fn run_parity_report() -> ParityReport {
    let base = run_abi_compliance();
    let edge = run_abi_edge_checks();
    let syscalls = [0u64, 1, 2, 3, 9, 11, 39, 41, 42, 44, 45, 202];
    let known = syscalls
        .iter()
        .filter(|n| linux_syscall_name(**n) != "unknown")
        .count();
    let network_class_ok = linux_syscall_class(41) == LinuxSyscallClass::Network;

    ParityReport {
        abi_base_ok: base.all_ok(),
        abi_edge_ok: edge.all_ok(),
        known_syscalls: known,
        network_class_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_report_ok() {
        assert!(run_parity_report().all_ok());
    }
}
