#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BpfLsmDecision {
    Allow,
    Deny,
}
pub fn hook_allows(prog_loaded: bool) -> BpfLsmDecision {
    if prog_loaded {
        BpfLsmDecision::Allow
    } else {
        BpfLsmDecision::Deny
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deny_without_prog() {
        assert_eq!(hook_allows(false), BpfLsmDecision::Deny);
    }
}
