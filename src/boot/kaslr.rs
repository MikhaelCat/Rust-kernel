#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaslrOffset(pub u64);
pub fn choose_offset(seed: u64) -> KaslrOffset {
    KaslrOffset((seed & 0xff) << 21)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kaslr_nonzero() {
        assert!(choose_offset(7).0 > 0);
    }
}
