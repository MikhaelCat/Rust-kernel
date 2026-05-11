#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entropy(pub u32);
pub fn mix(a: u32, b: u32) -> Entropy {
    Entropy(a ^ b.rotate_left(7))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mix_entropy() {
        assert_ne!(mix(1, 2).0, 0);
    }
}
