#[derive(Debug, Default)]
pub struct Iommu {
    mappings: u64,
}
impl Iommu {
    pub fn map(&mut self) {
        self.mappings += 1;
    }
    pub fn mappings(&self) -> u64 {
        self.mappings
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn map_iova() {
        let mut i = Iommu::default();
        i.map();
        assert_eq!(i.mappings(), 1);
    }
}
