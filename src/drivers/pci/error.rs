//! PCI Driver Error Types

#[derive(Debug, Clone)]
pub enum PciError {
    DeviceNotFound,
    BARMappingFailed,
    InvalidConfiguration,
    ResourceBusy,
    UnsupportedDevice,
}

impl std::fmt::Display for PciError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceNotFound => write!(f, "PCI device not found"),
            Self::BARMappingFailed => write!(f, "BAR mapping failed"),
            Self::InvalidConfiguration => write!(f, "Invalid PCI configuration"),
            Self::ResourceBusy => write!(f, "Resource busy"),
            Self::UnsupportedDevice => write!(f, "Unsupported PCI device"),
        }
    }
}

impl std::error::Error for PciError {}
