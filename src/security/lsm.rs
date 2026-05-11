#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmDecision {
    Allow,
    Deny,
}
pub fn check_ptrace(is_root: bool) -> LsmDecision {
    if is_root {
        LsmDecision::Allow
    } else {
        LsmDecision::Deny
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ptrace_policy() {
        assert_eq!(check_ptrace(false), LsmDecision::Deny);
    }
}
