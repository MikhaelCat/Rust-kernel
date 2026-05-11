use super::manager::Vfs;
use super::tmpfs::TmpFs;

#[derive(Debug, Default)]
pub struct FsSystem {
    vfs: Vfs,
    tmpfs: TmpFs,
}

impl FsSystem {
    pub fn new() -> Self {
        Self {
            vfs: Vfs::new(),
            tmpfs: TmpFs::default(),
        }
    }

    pub fn bootstrap(&mut self) {
        let _ = self.vfs.mkdir("/tmp");
        let _ = self.vfs.mount("/tmp", "tmpfs");
    }

    pub fn vfs(&self) -> &Vfs {
        &self.vfs
    }

    pub fn tmpfs_create_and_sync(&mut self, path: &str, data: &[u8]) -> bool {
        if self.vfs.create_file(path, data).is_err() {
            return false;
        }
        self.tmpfs.sync_from_vfs_file(path, data);
        true
    }

    pub fn tmpfs_read(&self, path: &str) -> Option<Vec<u8>> {
        self.tmpfs.read_file(path)
    }

    pub fn create_file(&mut self, path: &str, data: &[u8]) -> bool {
        self.vfs.create_file(path, data).is_ok()
    }

    pub fn read_file(&self, path: &str) -> Option<Vec<u8>> {
        self.vfs.read_file(path).ok()
    }

    pub fn write_file(&mut self, path: &str, data: &[u8]) -> bool {
        self.vfs.write_file(path, data).is_ok()
    }

    pub fn remove_file(&mut self, path: &str) -> bool {
        self.vfs.unlink(path).is_ok()
    }

    pub fn rename(&mut self, old: &str, new: &str) -> bool {
        self.vfs.rename(old, new).is_ok()
    }

    pub fn list_dir(&self, path: &str) -> Option<Vec<String>> {
        self.vfs.list_dir(path).ok()
    }

    pub fn set_quota_limits(&mut self, bytes: u64, inodes: u64) {
        self.vfs.set_quota_limit(bytes);
        self.vfs.set_inode_quota_limit(inodes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fs_bootstrap_mounts_tmp() {
        let mut fs = FsSystem::new();
        fs.bootstrap();
        assert!(fs.vfs().is_mounted("/tmp"));
    }

    #[test]
    fn tmpfs_sync_flow() {
        let mut fs = FsSystem::new();
        fs.bootstrap();
        assert!(fs.tmpfs_create_and_sync("/tmp/hello", b"world"));
        assert_eq!(fs.tmpfs_read("/tmp/hello").as_deref(), Some(&b"world"[..]));
    }

    #[test]
    fn fs_facade_flow() {
        let mut fs = FsSystem::new();
        fs.bootstrap();
        fs.set_quota_limits(1024, 16);
        assert!(fs.create_file("/tmp/f", b"abc"));
        assert_eq!(fs.read_file("/tmp/f").as_deref(), Some(&b"abc"[..]));
        assert!(fs.write_file("/tmp/f", b"xyz"));
        assert!(fs.rename("/tmp/f", "/tmp/f2"));
        assert_eq!(fs.read_file("/tmp/f2").as_deref(), Some(&b"xyz"[..]));
        let ls = fs.list_dir("/tmp").expect("list");
        assert!(ls.contains(&"f2".to_string()));
        assert!(fs.remove_file("/tmp/f2"));
    }
}
