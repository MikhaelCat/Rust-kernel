use super::hash::simple_hash32;
use super::signature::Signature;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoPolicyResult {
    Trusted,
    Rejected,
}

pub fn verify_artifact(data: &[u8], sig: Signature) -> CryptoPolicyResult {
    let digest = simple_hash32(data);
    if sig.verify(digest) {
        CryptoPolicyResult::Trusted
    } else {
        CryptoPolicyResult::Rejected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::hash::simple_hash32;

    #[test]
    fn policy_trusts_matching_signature() {
        let data = b"kernel-image";
        let sig = Signature::new(simple_hash32(data));
        assert_eq!(verify_artifact(data, sig), CryptoPolicyResult::Trusted);
    }
}
