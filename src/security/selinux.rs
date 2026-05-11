#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvDecision {
    Allow,
    Deny,
}
pub fn check_file_exec(is_labeled: bool) -> AvDecision {
    if is_labeled {
        AvDecision::Allow
    } else {
        AvDecision::Deny
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deny_unlabeled() {
        assert_eq!(check_file_exec(false), AvDecision::Deny);
    }
}
