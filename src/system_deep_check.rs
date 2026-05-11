use crate::drivers::lifecycle::{DriverLifecycle, DriverState};
use crate::mm::cow::CowTracker;
use crate::mm::fault::{FaultAction, PageFault, handle_fault};
use crate::virt::recovery::{VmFault, VmRecoveryAction, recover};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepCheckReport {
    pub mm_fault_ok: bool,
    pub mm_cow_ok: bool,
    pub driver_lifecycle_ok: bool,
    pub virt_recovery_ok: bool,
}

impl DeepCheckReport {
    pub fn all_ok(&self) -> bool {
        self.mm_fault_ok && self.mm_cow_ok && self.driver_lifecycle_ok && self.virt_recovery_ok
    }
}

pub fn run_deep_check() -> DeepCheckReport {
    let mm_fault_ok = handle_fault(PageFault::NotPresent) == FaultAction::MapPage
        && handle_fault(PageFault::Protection) == FaultAction::KillTask;

    let mut cow = CowTracker::default();
    cow.on_write_shared_page();
    let mm_cow_ok = cow.copies() == 1;

    let mut dl = DriverLifecycle::default();
    dl.probe();
    dl.bind();
    dl.suspend();
    dl.remove();
    let driver_lifecycle_ok = dl.state() == Some(DriverState::Removed);

    let virt_recovery_ok = recover(VmFault::Recoverable) == VmRecoveryAction::Restart
        && recover(VmFault::Fatal) == VmRecoveryAction::Destroy;

    DeepCheckReport {
        mm_fault_ok,
        mm_cow_ok,
        driver_lifecycle_ok,
        virt_recovery_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deep_check_passes() {
        assert!(run_deep_check().all_ok());
    }
}
