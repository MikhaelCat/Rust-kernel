use super::{
    component_01::MmComponent01, component_02::MmComponent02, component_03::MmComponent03,
    component_04::MmComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmComponentCatalog {
    pub enabled_count: usize,
}

impl MmComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = MmComponent01::new("phys");
        let c2 = MmComponent02::new("pager");
        let c3 = MmComponent03::new("vm");
        let c4 = MmComponent04::new("slab");
        let enabled = [c1.enabled, c2.enabled, c3.enabled, c4.enabled]
            .iter()
            .filter(|&&x| x)
            .count();
        Self {
            enabled_count: enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mm_catalog_sane() {
        assert_eq!(MmComponentCatalog::from_defaults().enabled_count, 4);
    }
}
