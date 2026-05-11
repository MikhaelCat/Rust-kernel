use super::hash::HashDigest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    pub digest: HashDigest,
}

impl Signature {
    pub fn new(digest: HashDigest) -> Self {
        Self { digest }
    }

    pub fn verify(&self, digest: HashDigest) -> bool {
        self.digest == digest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_verify() {
        let s = Signature::new(HashDigest(42));
        assert!(s.verify(HashDigest(42)));
        assert!(!s.verify(HashDigest(7)));
    }
}
