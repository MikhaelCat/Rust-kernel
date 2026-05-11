#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YamaScope {
    Disabled,
    Restricted,
}
pub fn ptrace_scope(strict: bool) -> YamaScope {
    if strict {
        YamaScope::Restricted
    } else {
        YamaScope::Disabled
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strict_scope() {
        assert_eq!(ptrace_scope(true), YamaScope::Restricted);
    }
}
