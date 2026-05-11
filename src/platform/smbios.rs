#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmbiosInfo {
    pub vendor: String,
}
impl SmbiosInfo {
    pub fn mock() -> Self {
        Self {
            vendor: "rust-linux".to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn smbios_mock() {
        assert_eq!(SmbiosInfo::mock().vendor, "rust-linux");
    }
}
