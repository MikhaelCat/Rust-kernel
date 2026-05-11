use super::{
    component_01::PowerComponent01, component_02::PowerComponent02, component_03::PowerComponent03,
    component_04::PowerComponent04, component_05::PowerComponent05, component_06::PowerComponent06,
    component_07::PowerComponent07, component_08::PowerComponent08, component_09::PowerComponent09,
    component_10::PowerComponent10,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerComponentCatalog {
    pub static_components: [bool; 4],
    pub dynamic_ids: [u32; 6],
}

impl PowerComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = PowerComponent01::new("cpuidle");
        let c2 = PowerComponent02::new("cpufreq");
        let c3 = PowerComponent03::new("thermal");
        let c4 = PowerComponent04::new("regulator");

        let i5 = PowerComponent05::new(5).id;
        let i6 = PowerComponent06::new(6).id;
        let i7 = PowerComponent07::new(7).id;
        let i8 = PowerComponent08::new(8).id;
        let i9 = PowerComponent09::new(9).id;
        let i10 = PowerComponent10::new(10).id;

        Self {
            static_components: [c1.enabled, c2.enabled, c3.enabled, c4.enabled],
            dynamic_ids: [i5, i6, i7, i8, i9, i10],
        }
    }

    pub fn static_enabled_count(&self) -> usize {
        self.static_components.iter().filter(|&&x| x).count()
    }

    pub fn all_dynamic_unique(&self) -> bool {
        let mut ids = self.dynamic_ids;
        ids.sort_unstable();
        ids.windows(2).all(|w| w[0] != w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_sane() {
        let cat = PowerComponentCatalog::from_defaults();
        assert_eq!(cat.static_enabled_count(), 4);
        assert!(cat.all_dynamic_unique());
    }
}
