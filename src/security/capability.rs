#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    SysAdmin,
    NetAdmin,
}
pub fn has_cap(is_root: bool, _cap: Capability) -> bool {
    is_root
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cap_check() {
        assert!(has_cap(true, Capability::SysAdmin));
        assert!(!has_cap(false, Capability::NetAdmin));
    }
}
