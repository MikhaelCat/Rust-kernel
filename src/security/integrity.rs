#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Integrity {
    Trusted,
    Untrusted,
}
pub fn appraise(sig_ok: bool) -> Integrity {
    if sig_ok {
        Integrity::Trusted
    } else {
        Integrity::Untrusted
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn appraisal() {
        assert_eq!(appraise(false), Integrity::Untrusted);
    }
}
