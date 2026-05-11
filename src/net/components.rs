use super::{
    component_01::NetComponent01, component_02::NetComponent02, component_03::NetComponent03,
    component_04::NetComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetComponentCatalog {
    pub enabled_count: usize,
}

impl NetComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = NetComponent01::new("device");
        let c2 = NetComponent02::new("route");
        let c3 = NetComponent03::new("socket");
        let c4 = NetComponent04::new("packet");
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
    fn net_catalog_sane() {
        assert_eq!(NetComponentCatalog::from_defaults().enabled_count, 4);
    }
}
