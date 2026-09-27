//! Hugepage error types for Linux kernel memory management

/// Error codes for hugepage operations
#[derive(Debug, Clone, Copy)]
pub enum HugePageError {
    InvalidSize,
    AllocationFailed,
    MappingError,
    UnsupportedOperation,
}

impl std::fmt::Display for HugePageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HugePageError::InvalidSize => write!(f, "Invalid huge page size"),
            HugePageError::AllocationFailed => write!(f, "Huge page allocation failed"),
            HugePageError::MappingError => write!(f, "Huge page mapping error"),
            HugePageError::UnsupportedOperation => write!(f, "Unsupported operation"),
        }
    }
}

impl std::error::Error for HugePageError {}
