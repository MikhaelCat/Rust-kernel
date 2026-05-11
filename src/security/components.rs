use super::{
    component_01::SecurityComponent01, component_02::SecurityComponent02,
    component_03::SecurityComponent03, component_04::SecurityComponent04,
    component_05::SecurityComponent05, component_06::SecurityComponent06,
    component_07::SecurityComponent07, component_08::SecurityComponent08,
    component_09::SecurityComponent09, component_10::SecurityComponent10,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityComponentCatalog {
    pub policy_components: [bool; 4],
    pub hook_ids: [u32; 6],
}

impl SecurityComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = SecurityComponent01::new("selinux");
        let c2 = SecurityComponent02::new("apparmor");
        let c3 = SecurityComponent03::new("landlock");
        let c4 = SecurityComponent04::new("seccomp");

        Self {
            policy_components: [c1.enabled, c2.enabled, c3.enabled, c4.enabled],
            hook_ids: [
                SecurityComponent05::new(5).id,
                SecurityComponent06::new(6).id,
                SecurityComponent07::new(7).id,
                SecurityComponent08::new(8).id,
                SecurityComponent09::new(9).id,
                SecurityComponent10::new(10).id,
            ],
        }
    }

    pub fn policy_enabled_count(&self) -> usize {
        self.policy_components.iter().filter(|&&x| x).count()
    }

    pub fn hooks_span_expected_range(&self) -> bool {
        self.hook_ids.iter().min() == Some(&5) && self.hook_ids.iter().max() == Some(&10)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_sane() {
        let cat = SecurityComponentCatalog::from_defaults();
        assert_eq!(cat.policy_enabled_count(), 4);
        assert!(cat.hooks_span_expected_range());
    }
}
