//! VFS Error Types

#[derive(Debug, Clone)]
pub enum VfsError {
    NotFound,
    PermissionDenied,
    IoError,
    AlreadyExists,
    InvalidPath,
    FileTooLarge,
    NoSpaceLeft,
}

impl std::fmt::Display for VfsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "Not found"),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::IoError => write!(f, "I/O error"),
            Self::AlreadyExists => write!(f, "Already exists"),
            Self::InvalidPath => write!(f, "Invalid path"),
            Self::FileTooLarge => write!(f, "File too large"),
            Self::NoSpaceLeft => write!(f, "No space left"),
        }
    }
}

impl std::error::Error for VfsError {}
