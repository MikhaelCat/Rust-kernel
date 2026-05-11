#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AaDecision {
    Allow,
    Deny,
}
pub fn check_path(path: &str) -> AaDecision {
    if path.starts_with("/home/") {
        AaDecision::Allow
    } else {
        AaDecision::Deny
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deny_root_write() {
        assert_eq!(check_path("/etc/shadow"), AaDecision::Deny);
    }
}
