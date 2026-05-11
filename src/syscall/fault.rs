use super::policy::{SysPolicyDecision, check_syscall_allowed};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallFault {
    PolicyDenied,
    UnknownNumber,
}

pub fn syscall_precheck(num: u64, privileged: bool) -> Result<(), SyscallFault> {
    match check_syscall_allowed(num, privileged) {
        SysPolicyDecision::Allow => Ok(()),
        SysPolicyDecision::Deny => Err(SyscallFault::PolicyDenied),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precheck_denies_unprivileged_mount() {
        assert_eq!(
            syscall_precheck(165, false),
            Err(SyscallFault::PolicyDenied)
        );
        assert_eq!(syscall_precheck(39, false), Ok(()));
    }
}
