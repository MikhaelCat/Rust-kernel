#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsHealth {
    pub mounted_root: bool,
}

pub fn check_root(mounted_root: bool) -> FsHealth {
    FsHealth { mounted_root }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_mount_health() {
        assert!(check_root(true).mounted_root);
    }
}
