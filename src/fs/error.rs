#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    AlreadyExists,
    InvalidPath,
    NotADirectory,
    IsDirectory,
    MountPointBusy,
    QuotaExceeded,
    PermissionDenied,
}
