use super::{
    component_01::SyscallComponent01, component_02::SyscallComponent02,
    component_03::SyscallComponent03, component_04::SyscallComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyscallComponentCatalog {
    pub enabled_count: usize,
}

impl SyscallComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = SyscallComponent01::new("table");
        let c2 = SyscallComponent02::new("dispatch");
        let c3 = SyscallComponent03::new("abi");
        let c4 = SyscallComponent04::new("errno");
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
    fn syscall_catalog_sane() {
        assert_eq!(SyscallComponentCatalog::from_defaults().enabled_count, 4);
    }
}
