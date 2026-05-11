use super::{
    component_01::IpcComponent01, component_02::IpcComponent02, component_03::IpcComponent03,
    component_04::IpcComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcComponentCatalog {
    pub enabled_count: usize,
}

impl IpcComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = IpcComponent01::new("queue");
        let c2 = IpcComponent02::new("pipe");
        let c3 = IpcComponent03::new("futex");
        let c4 = IpcComponent04::new("signal");
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
    fn ipc_catalog_sane() {
        assert_eq!(IpcComponentCatalog::from_defaults().enabled_count, 4);
    }
}
