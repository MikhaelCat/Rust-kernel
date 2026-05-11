use super::{
    component_01::TimeComponent01, component_02::TimeComponent02, component_03::TimeComponent03,
    component_04::TimeComponent04, component_05::TimeComponent05, component_06::TimeComponent06,
    component_07::TimeComponent07, component_08::TimeComponent08, component_09::TimeComponent09,
    component_10::TimeComponent10,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeComponentCatalog {
    pub base_components: [bool; 4],
    pub feature_ids: [u32; 6],
}

impl TimeComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = TimeComponent01::new("tick");
        let c2 = TimeComponent02::new("hrtimer");
        let c3 = TimeComponent03::new("clockevent");
        let c4 = TimeComponent04::new("ntp");

        Self {
            base_components: [c1.enabled, c2.enabled, c3.enabled, c4.enabled],
            feature_ids: [
                TimeComponent05::new(5).id,
                TimeComponent06::new(6).id,
                TimeComponent07::new(7).id,
                TimeComponent08::new(8).id,
                TimeComponent09::new(9).id,
                TimeComponent10::new(10).id,
            ],
        }
    }

    pub fn base_enabled_count(&self) -> usize {
        self.base_components.iter().filter(|&&x| x).count()
    }

    pub fn max_feature_id(&self) -> u32 {
        *self.feature_ids.iter().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_sane() {
        let cat = TimeComponentCatalog::from_defaults();
        assert_eq!(cat.base_enabled_count(), 4);
        assert_eq!(cat.max_feature_id(), 10);
    }
}
