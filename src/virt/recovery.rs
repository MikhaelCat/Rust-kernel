#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmFault {
    Recoverable,
    Fatal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmRecoveryAction {
    Restart,
    Destroy,
}

pub fn recover(f: VmFault) -> VmRecoveryAction {
    match f {
        VmFault::Recoverable => VmRecoveryAction::Restart,
        VmFault::Fatal => VmRecoveryAction::Destroy,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_recovery_actions() {
        assert_eq!(recover(VmFault::Recoverable), VmRecoveryAction::Restart);
        assert_eq!(recover(VmFault::Fatal), VmRecoveryAction::Destroy);
    }
}
