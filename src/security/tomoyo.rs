#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TomoyoDecision {
    Allow,
    Deny,
}
pub fn check_exec(path: &str) -> TomoyoDecision {
    if path.starts_with("/usr/bin/") {
        TomoyoDecision::Allow
    } else {
        TomoyoDecision::Deny
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deny_unknown_exec() {
        assert_eq!(check_exec("/tmp/x"), TomoyoDecision::Deny);
    }
}
