//! Linux Security Module (LSM) error handling

/// LSM-specific error codes
#[derive(Debug, Clone, Copy)]
pub enum LsmError {
    HookNotFound,
    PermissionDenied,
    InvalidCredential,
    SecurityViolation,
    ModuleLoadFailed,
}

impl std::fmt::Display for LsmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LsmError::HookNotFound => write!(f, "LSM hook not found"),
            LsmError::PermissionDenied => write!(f, "Permission denied by security policy"),
            LsmError::InvalidCredential => write!(f, "Invalid security credential"),
            LsmError::SecurityViolation => write!(f, "Security policy violation"),
            LsmError::ModuleLoadFailed => write!(f, "LSM module load failed"),
        }
    }
}

impl std::error::Error for LsmError {}
