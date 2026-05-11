use super::{
    component_01::KernelComponent01, component_02::KernelComponent02,
    component_03::KernelComponent03, component_04::KernelComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelComponentCatalog {
    pub enabled_count: usize,
}

impl KernelComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = KernelComponent01::new("sched");
        let c2 = KernelComponent02::new("irq");
        let c3 = KernelComponent03::new("timer");
        let c4 = KernelComponent04::new("process");
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
    fn kernel_catalog_sane() {
        assert_eq!(KernelComponentCatalog::from_defaults().enabled_count, 4);
    }
}
