#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountPoint {
    pub path: String,
    pub fs_type: String,
}

impl MountPoint {
    pub fn new(path: &str, fs_type: &str) -> Self {
        Self {
            path: path.to_string(),
            fs_type: fs_type.to_string(),
        }
    }
}
