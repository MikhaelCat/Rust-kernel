#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HugePage {
    pub order: u8,
}
impl HugePage {
    pub fn size_bytes(&self) -> usize {
        4096usize << self.order
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hugepage_size() {
        assert_eq!(HugePage { order: 9 }.size_bytes(), 2 * 1024 * 1024);
    }
}
