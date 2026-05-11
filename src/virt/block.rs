#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtBlock {
    pub sectors: u64,
}

impl VirtBlock {
    pub fn new(sectors: u64) -> Self {
        Self { sectors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_block() {
        let b = VirtBlock::new(1_000_000);
        assert_eq!(b.sectors, 1_000_000);
    }
}
