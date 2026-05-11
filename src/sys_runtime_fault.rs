use crate::drivers::fault::DriverFault;
use crate::drivers::recovery::recover_from_fault;
use crate::net::fault::{NetFault, NetFaultAction, handle_net_fault};
use crate::syscall::fault::{SyscallFault, syscall_precheck};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeFaultReport {
    pub driver_recovered: bool,
    pub net_action: NetFaultAction,
    pub syscall_ok: bool,
}

pub fn run_fault_path() -> RuntimeFaultReport {
    let drv = recover_from_fault(DriverFault::Recoverable);
    let net_action = handle_net_fault(NetFault::RouteMissing);
    let syscall_ok = syscall_precheck(39, false).is_ok()
        && syscall_precheck(165, false) == Err(SyscallFault::PolicyDenied);

    RuntimeFaultReport {
        driver_recovered: drv.recovered,
        net_action,
        syscall_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fault_runtime_smoke() {
        let r = run_fault_path();
        assert!(r.driver_recovered);
        assert_eq!(r.net_action, NetFaultAction::Reconfigure);
        assert!(r.syscall_ok);
    }
}
