//! SELinux security module error handling

/// SELinux-specific error codes
#[derive(Debug, Clone, Copy)]
pub enum SelinuxError {
    PolicyLoadFailed,
    ContextNotFound,
    TransitionDenied,
    InvalidLabel,
    InitializationError,
}

impl std::fmt::Display for SelinuxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelinuxError::PolicyLoadFailed => write!(f, "SELinux policy load failed"),
            SelinuxError::ContextNotFound => write!(f, "Security context not found"),
            SelinuxError::TransitionDenied => write!(f, "Security transition denied"),
            SelinuxError::InvalidLabel => write!(f, "Invalid security label"),
            SelinuxError::InitializationError => write!(f, "SELinux initialization error"),
        }
    }
}

impl std::error::Error for SelinuxError {}
