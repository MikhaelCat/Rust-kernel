use crate::abi::{run_abi_compliance, run_abi_edge_checks};
use crate::boot::self_check::run_boot_self_check;
use crate::mm::protection::{AccessType, ProtFlags, check_access};
use crate::platform::self_check::run_platform_self_check;
use crate::sched::manager::{SchedClass, SchedulerManager};
use crate::security::bundle::run_security_bundle;
use crate::sys_runtime::run_runtime_path;
use crate::sys_runtime_fault::run_fault_path;
use crate::system_deep_check::run_deep_check;
use crate::system_parity::run_parity_report;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfCheckReport {
    pub runtime_ok: bool,
    pub boot_ok: bool,
    pub fault_ok: bool,
    pub abi_ok: bool,
    pub sched_ok: bool,
    pub platform_ok: bool,
    pub mm_prot_ok: bool,
    pub deep_ok: bool,
    pub security_bundle_ok: bool,
    pub report_ok: bool,
    pub parity_ok: bool,
}

impl SelfCheckReport {
    pub fn all_ok(&self) -> bool {
        self.runtime_ok
            && self.boot_ok
            && self.fault_ok
            && self.abi_ok
            && self.sched_ok
            && self.platform_ok
            && self.mm_prot_ok
            && self.deep_ok
            && self.security_bundle_ok
            && self.report_ok
            && self.parity_ok
    }
}

pub fn run_self_check() -> SelfCheckReport {
    let runtime = run_runtime_path();
    let boot_ok = run_boot_self_check();
    let fault = run_fault_path();
    let abi = run_abi_compliance();
    let abi_edge = run_abi_edge_checks();
    let platform_ok = run_platform_self_check();
    let deep_ok = run_deep_check().all_ok();
    let parity_ok = run_parity_report().all_ok();

    let runtime_ok = runtime.platform_ready
        && runtime.block_inflight == 1
        && runtime.io_completed
        && runtime.vm_started
        && runtime.vm_healthy
        && runtime.diagnostics_checks >= 1;

    let fault_ok = fault.driver_recovered && fault.syscall_ok;
    let abi_ok = abi.all_ok() && abi_edge.all_ok();

    let mut sched = SchedulerManager::default();
    sched.enqueue_with(10, SchedClass::Cfs, 120, 0);
    sched.enqueue_with(20, SchedClass::Rt, 10, 0);
    let sched_ok = sched.pick_next() == Ok(20) && sched.set_affinity(10, 1).is_ok();

    let p = ProtFlags::rw();
    let mm_prot_ok = check_access(p, AccessType::Read)
        && check_access(p, AccessType::Write)
        && !check_access(p, AccessType::Exec);

    let security_bundle_ok = run_security_bundle().all_ok();
    let report_ok = true;

    SelfCheckReport {
        runtime_ok,
        boot_ok,
        fault_ok,
        abi_ok,
        sched_ok,
        platform_ok,
        mm_prot_ok,
        deep_ok,
        security_bundle_ok,
        report_ok,
        parity_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn self_check_passes() {
        let r = run_self_check();
        assert!(r.all_ok());
    }
}
