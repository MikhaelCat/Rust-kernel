//! USB Driver Error Types

#[derive(Debug, Clone)]
pub enum UsbError {
    DeviceNotAttached,
    DescriptorParseFailed,
    EndpointNotFound,
    TransferFailed,
    Busy,
}

impl std::fmt::Display for UsbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeviceNotAttached => write!(f, "USB device not attached"),
            Self::DescriptorParseFailed => write!(f, "Descriptor parse failed"),
            Self::EndpointNotFound => write!(f, "Endpoint not found"),
            Self::TransferFailed => write!(f, "Transfer failed"),
            Self::Busy => write!(f, "Operation busy"),
        }
    }
}

impl std::error::Error for UsbError {}
