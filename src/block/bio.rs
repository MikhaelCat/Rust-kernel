#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bio {
    pub sector: u64,
    pub bytes: usize,
}
impl Bio {
    pub fn new(sector: u64, bytes: usize) -> Self {
        Self { sector, bytes }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bio_new() {
        assert_eq!(Bio::new(1, 512).bytes, 512);
    }
}
