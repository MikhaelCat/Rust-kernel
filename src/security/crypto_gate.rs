use crate::crypto::{CryptoPolicyResult, Signature, verify_artifact};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityGateResult {
    Allow,
    Deny,
}

pub fn allow_if_trusted(data: &[u8], sig: Signature) -> SecurityGateResult {
    match verify_artifact(data, sig) {
        CryptoPolicyResult::Trusted => SecurityGateResult::Allow,
        CryptoPolicyResult::Rejected => SecurityGateResult::Deny,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::{Signature, simple_hash32};

    #[test]
    fn gate_allows_trusted_artifact() {
        let data = b"module";
        let sig = Signature::new(simple_hash32(data));
        assert_eq!(allow_if_trusted(data, sig), SecurityGateResult::Allow);
    }
}
