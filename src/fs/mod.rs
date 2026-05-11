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
