#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashDigest(pub u32);

pub fn simple_hash32(data: &[u8]) -> HashDigest {
    // Deterministic lightweight hash placeholder for subsystem integration tests.
    let mut h: u32 = 0x811C9DC5;
    for b in data {
        h ^= *b as u32;
        h = h.wrapping_mul(0x01000193);
    }
    HashDigest(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic() {
        assert_eq!(simple_hash32(b"abc"), simple_hash32(b"abc"));
        assert_ne!(simple_hash32(b"abc"), simple_hash32(b"abd"));
    }
}
