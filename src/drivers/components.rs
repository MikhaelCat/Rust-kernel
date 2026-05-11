use super::{
    component_01::DriversComponent01, component_02::DriversComponent02,
    component_03::DriversComponent03, component_04::DriversComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriversComponentCatalog {
    pub enabled_count: usize,
}

impl DriversComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = DriversComponent01::new("registry");
        let c2 = DriversComponent02::new("bus");
        let c3 = DriversComponent03::new("probe");
        let c4 = DriversComponent04::new("irq");
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
    fn drivers_catalog_sane() {
        assert_eq!(DriversComponentCatalog::from_defaults().enabled_count, 4);
    }
}
