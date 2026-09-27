//! PCI error handling
#[derive(Debug, Clone)]
pub struct PciError(pub &'static str);

impl std::fmt::Display for PciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for PciError {}
