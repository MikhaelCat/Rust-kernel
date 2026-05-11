use super::fault::{DriverFault, FaultAction, handle_fault};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryReport {
    pub action: FaultAction,
    pub recovered: bool,
}

pub fn recover_from_fault(fault: DriverFault) -> RecoveryReport {
    let action = handle_fault(fault);
    let recovered = matches!(action, FaultAction::Retry | FaultAction::Reset);
    RecoveryReport { action, recovered }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recoverable_fault_recovers() {
        let r = recover_from_fault(DriverFault::Recoverable);
        assert_eq!(r.action, FaultAction::Retry);
        assert!(r.recovered);
    }

    #[test]
    fn fatal_fault_quarantines() {
        let r = recover_from_fault(DriverFault::Fatal);
        assert_eq!(r.action, FaultAction::Quarantine);
        assert!(!r.recovered);
    }
}
