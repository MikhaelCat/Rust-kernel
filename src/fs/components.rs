use super::{
    component_01::FsComponent01, component_02::FsComponent02, component_03::FsComponent03,
    component_04::FsComponent04,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsComponentCatalog {
    pub enabled_count: usize,
}

impl FsComponentCatalog {
    pub fn from_defaults() -> Self {
        let c1 = FsComponent01::new("inode");
        let c2 = FsComponent02::new("dentry");
        let c3 = FsComponent03::new("vfs");
        let c4 = FsComponent04::new("mount");
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
    fn fs_catalog_sane() {
        assert_eq!(FsComponentCatalog::from_defaults().enabled_count, 4);
    }
}
