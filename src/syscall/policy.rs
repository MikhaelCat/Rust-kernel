#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SysPolicyDecision {
    Allow,
    Deny,
}

pub fn check_syscall_allowed(num: u64, privileged: bool) -> SysPolicyDecision {
    // Example policy: mount-like op numbers require privilege.
    if num == 165 || num == 166 {
        if privileged {
            SysPolicyDecision::Allow
        } else {
            SysPolicyDecision::Deny
        }
    } else {
        SysPolicyDecision::Allow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privileged_gate() {
        assert_eq!(check_syscall_allowed(165, false), SysPolicyDecision::Deny);
        assert_eq!(check_syscall_allowed(165, true), SysPolicyDecision::Allow);
        assert_eq!(check_syscall_allowed(39, false), SysPolicyDecision::Allow);
    }
}
