//! File System VFS for Linux Kernel on Rust

pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;

pub mod dentry;
pub mod error;
pub mod file;
pub mod inode;
pub mod manager;
pub mod mount;
pub mod vfs;

pub use error::FsError;
pub use file::File;
pub use manager::Vfs;
pub use mount::MountPoint;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_vfs_flow() {
        let mut fs = Vfs::new();
        fs.mkdir("/tmp").expect("mkdir failed");
        fs.create_file("/tmp/a", b"a").expect("create failed");
        assert_eq!(fs.read_file("/tmp/a").expect("read failed"), b"a");
    }
}

pub mod dcache;
pub mod exportfs;
pub mod journal;
pub mod overlay;
pub mod page_cache;
pub mod quota;
pub mod tmpfs;

pub mod status;
pub mod system;
pub use system::FsSystem;
pub mod health;
pub mod system2;
pub mod components;
pub mod perm;
pub mod inode_ops;
pub mod ownership;

#[derive(Debug, Clone)]
pub struct FsStats {
    pub total_inodes: u64,
    pub used_inodes: u64,
    pub total_bytes: u64,
    pub used_bytes: u64,
}

impl Default for FsStats {
    fn default() -> Self {
        Self {
            total_inodes: 0,
            used_inodes: 0,
            total_bytes: 0,
            used_bytes: 0,
        }
    }
}

impl FsStats {
    pub fn inode_utilization(&self) -> f64 {
        if self.total_inodes == 0 { return 0.0; }
        (self.used_inodes as f64 / self.total_inodes as f64) * 100.0
    }

    pub fn space_utilization(&self) -> f64 {
        if self.total_bytes == 0 { return 0.0; }
        (self.used_bytes as f64 / self.total_bytes as f64) * 100.0
    }
}

#[derive(Debug, Clone)]
pub struct Ext4Stats {
    pub inodes_free: u64,
    pub blocks_free: u64,
    pub open_files: u32,
    pub journal_entries: u64,
}

#[derive(Debug, Clone)]
pub struct XfsStats {
    pub allocation_groups: u32,
    pub extent_tree_nodes: u64,
    pub delayed_allocs: u32,
}
