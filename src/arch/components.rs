use super::{
    component_01::ArchComponent01, component_02::ArchComponent02, component_03::ArchComponent03,
    component_04::ArchComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchComponentCatalog {
    pub enabled_count: usize,
}

impl ArchComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = ArchComponent01::new("cpu");
        let c2 = ArchComponent02::new("idt");
        let c3 = ArchComponent03::new("paging");
        let c4 = ArchComponent04::new("smp");
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
    fn arch_catalog_sane() {
        assert_eq!(ArchComponentCatalog::from_defaults().enabled_count, 4);
    }
}
