#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LandlockDecision {
    Allow,
    Deny,
}
pub fn check_write(path: &str) -> LandlockDecision {
    if path.starts_with("/tmp/") {
        LandlockDecision::Allow
    } else {
        LandlockDecision::Deny
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn landlock_policy() {
        assert_eq!(check_write("/etc/passwd"), LandlockDecision::Deny);
    }
}
