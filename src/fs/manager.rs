use std::collections::{BTreeMap, BTreeSet};

use super::dentry::Dentry;
use super::error::FsError;
use super::file::File;
use super::inode::{Inode, InodeKind};
use super::inode_ops::InodeOps;
use super::journal::{Journal, JournalEntry};
use super::mount::MountPoint;
use super::ownership::{Owner, OwnershipTable};
use super::page_cache::PageCache;
use super::perm::{Cred, Mode, can_read, can_write};
use super::quota::Quota;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStat {
    pub ino: u64,
    pub size: u64,
    pub owner: Owner,
    pub mode: Mode,
    pub nlink: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VfsSnapshot {
    pub files: BTreeMap<String, Vec<u8>>,
    pub directories: BTreeSet<String>,
    pub symlinks: BTreeMap<String, String>,
}

#[derive(Debug, Default)]
pub struct Vfs {
    next_ino: u64,
    inodes: BTreeMap<u64, Inode>,
    dentries: BTreeMap<String, Dentry>,
    files: BTreeMap<String, Vec<u8>>,
    directories: BTreeSet<String>,
    mounts: BTreeMap<String, MountPoint>,
    inode_ops: InodeOps,
    ownership: OwnershipTable,
    modes: BTreeMap<String, Mode>,
    quota: Quota,
    page_cache: PageCache,
    symlinks: BTreeMap<String, String>,
    open_files: BTreeMap<i64, File>,
    next_fd: i64,
    journal: Journal,
}

impl Vfs {
    pub fn new() -> Self {
        let mut vfs = Self {
            next_ino: 2,
            inodes: BTreeMap::new(),
            dentries: BTreeMap::new(),
            files: BTreeMap::new(),
            directories: BTreeSet::new(),
            mounts: BTreeMap::new(),
            inode_ops: InodeOps::default(),
            ownership: OwnershipTable::default(),
            modes: BTreeMap::new(),
            quota: Quota::default(),
            page_cache: PageCache::default(),
            symlinks: BTreeMap::new(),
            open_files: BTreeMap::new(),
            next_fd: 3,
            journal: Journal::default(),
        };
        vfs.directories.insert("/".to_string());
        vfs.inodes.insert(1, Inode::new(1, InodeKind::Directory));
        vfs.inode_ops.create_inode(1);
        vfs.ownership.chown("/", 0, 0);
        vfs.modes.insert(
            "/".to_string(),
            Mode {
                owner_read: true,
                owner_write: true,
                other_read: true,
                other_write: false,
            },
        );
        vfs.quota.set_limit(1024 * 1024 * 128);
        vfs.quota.set_inode_limit(1024 * 1024);
        vfs
    }

    fn alloc_ino(&mut self, kind: InodeKind) -> u64 {
        let ino = self.next_ino;
        self.next_ino += 1;
        self.inodes.insert(ino, Inode::new(ino, kind));
        self.inode_ops.create_inode(ino);
        ino
    }

    fn validate_abs_path(path: &str) -> Result<(), FsError> {
        if path.starts_with('/') && path.len() >= 2 {
            Ok(())
        } else {
            Err(FsError::InvalidPath)
        }
    }

    fn parent_dir(path: &str) -> &str {
        match path.rfind('/') {
            Some(0) => "/",
            Some(idx) => &path[..idx],
            None => "",
        }
    }

    fn resolve_path<'a>(&'a self, path: &'a str) -> &'a str {
        self.symlinks.get(path).map(String::as_str).unwrap_or(path)
    }

    pub fn mkdir(&mut self, path: &str) -> Result<(), FsError> {
        Self::validate_abs_path(path)?;
        if self.files.contains_key(path) || self.directories.contains(path) {
            return Err(FsError::AlreadyExists);
        }

        let parent = Self::parent_dir(path);
        if !self.directories.contains(parent) {
            return Err(FsError::NotFound);
        }

        let ino = self.alloc_ino(InodeKind::Directory);
        self.dentries
            .insert(path.to_string(), Dentry::new(path, ino));
        self.directories.insert(path.to_string());
        self.journal.record("mkdir", path);
        self.ownership.chown(path, 0, 0);
        self.modes.insert(
            path.to_string(),
            Mode {
                owner_read: true,
                owner_write: true,
                other_read: true,
                other_write: false,
            },
        );
        Ok(())
    }

    pub fn create_file(&mut self, path: &str, content: &[u8]) -> Result<(), FsError> {
        self.create_file_as(path, content, Cred { uid: 0 }, 0, 0)
    }

    pub fn create_file_as(
        &mut self,
        path: &str,
        content: &[u8],
        cred: Cred,
        uid: u32,
        gid: u32,
    ) -> Result<(), FsError> {
        Self::validate_abs_path(path)?;
        if self.files.contains_key(path) || self.directories.contains(path) {
            return Err(FsError::AlreadyExists);
        }

        let parent = Self::parent_dir(path);
        if !self.directories.contains(parent) {
            return Err(FsError::NotFound);
        }
        let parent_owner = self
            .ownership
            .owner(parent)
            .unwrap_or(Owner { uid: 0, gid: 0 });
        let parent_mode = self.modes.get(parent).copied().unwrap_or(Mode {
            owner_read: true,
            owner_write: true,
            other_read: true,
            other_write: false,
        });
        if !can_write(cred, parent_owner.uid, parent_mode) {
            return Err(FsError::PermissionDenied);
        }
        if !self.quota.reserve(content.len() as u64) {
            return Err(FsError::QuotaExceeded);
        }
        if !self.quota.reserve_inode() {
            return Err(FsError::QuotaExceeded);
        }

        let ino = self.alloc_ino(InodeKind::File);
        self.dentries
            .insert(path.to_string(), Dentry::new(path, ino));
        self.files.insert(path.to_string(), content.to_vec());
        if let Some(inode) = self.inodes.get_mut(&ino) {
            inode.size = content.len() as u64;
        }
        self.ownership.chown(path, uid, gid);
        self.modes.insert(
            path.to_string(),
            Mode {
                owner_read: true,
                owner_write: true,
                other_read: true,
                other_write: false,
            },
        );
        self.page_cache.put(ino, content);
        self.journal.record("create", path);
        Ok(())
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, FsError> {
        self.read_file_as(path, Cred { uid: 0 })
    }

    pub fn read_file_as(&self, path: &str, cred: Cred) -> Result<Vec<u8>, FsError> {
        let path = self.resolve_path(path);
        if self.directories.contains(path) {
            return Err(FsError::IsDirectory);
        }
        let d = self.dentries.get(path).ok_or(FsError::NotFound)?;
        let owner = self
            .ownership
            .owner(path)
            .unwrap_or(Owner { uid: 0, gid: 0 });
        let mode = self.modes.get(path).copied().unwrap_or(Mode {
            owner_read: true,
            owner_write: true,
            other_read: true,
            other_write: false,
        });
        if !can_read(cred, owner.uid, mode) {
            return Err(FsError::PermissionDenied);
        }
        if let Some(buf) = self.page_cache.get(d.inode) {
            return Ok(buf.to_vec());
        }
        self.files.get(path).cloned().ok_or(FsError::NotFound)
    }

    pub fn write_file(&mut self, path: &str, content: &[u8]) -> Result<(), FsError> {
        self.write_file_as(path, content, Cred { uid: 0 })
    }

    pub fn write_file_as(&mut self, path: &str, content: &[u8], cred: Cred) -> Result<(), FsError> {
        let path = self.resolve_path(path).to_string();
        let path = path.as_str();
        if self.directories.contains(path) {
            return Err(FsError::IsDirectory);
        }
        let owner = self
            .ownership
            .owner(path)
            .unwrap_or(Owner { uid: 0, gid: 0 });
        let mode = self.modes.get(path).copied().unwrap_or(Mode {
            owner_read: true,
            owner_write: true,
            other_read: true,
            other_write: false,
        });
        if !can_write(cred, owner.uid, mode) {
            return Err(FsError::PermissionDenied);
        }

        let buf = self.files.get_mut(path).ok_or(FsError::NotFound)?;
        *buf = content.to_vec();

        if let Some(d) = self.dentries.get(path)
            && let Some(inode) = self.inodes.get_mut(&d.inode)
        {
            inode.size = content.len() as u64;
            self.page_cache.put(d.inode, content);
        }
        self.journal.record("write", path);
        Ok(())
    }

    pub fn unlink(&mut self, path: &str) -> Result<(), FsError> {
        if self.symlinks.remove(path).is_some() {
            return Ok(());
        }
        if self.directories.contains(path) {
            return Err(FsError::NotADirectory);
        }
        let d = self.dentries.remove(path).ok_or(FsError::NotFound)?;
        self.files.remove(path).ok_or(FsError::NotFound)?;
        let _ = self.inode_ops.unlink(d.inode);
        self.page_cache.drop_inode(d.inode);
        self.quota.release_inode();
        self.ownership.chown(path, 0, 0);
        self.modes.remove(path);
        self.journal.record("unlink", path);
        Ok(())
    }

    pub fn rmdir(&mut self, path: &str) -> Result<(), FsError> {
        if path == "/" {
            return Err(FsError::InvalidPath);
        }
        if !self.directories.contains(path) {
            return Err(FsError::NotFound);
        }
        let prefix = format!("{path}/");
        if self.files.keys().any(|p| p.starts_with(&prefix))
            || self
                .directories
                .iter()
                .any(|p| p.starts_with(&prefix) && p != path)
            || self.symlinks.keys().any(|p| p.starts_with(&prefix))
        {
            return Err(FsError::MountPointBusy);
        }
        if self.mounts.contains_key(path) {
            return Err(FsError::MountPointBusy);
        }
        self.directories.remove(path);
        self.dentries.remove(path);
        self.modes.remove(path);
        self.journal.record("rmdir", path);
        Ok(())
    }

    pub fn chmod(&mut self, path: &str, mode: Mode) -> Result<(), FsError> {
        if !self.files.contains_key(path) && !self.directories.contains(path) {
            return Err(FsError::NotFound);
        }
        self.modes.insert(path.to_string(), mode);
        Ok(())
    }

    pub fn chown(&mut self, path: &str, uid: u32, gid: u32) -> Result<(), FsError> {
        if !self.files.contains_key(path) && !self.directories.contains(path) {
            return Err(FsError::NotFound);
        }
        self.ownership.chown(path, uid, gid);
        Ok(())
    }

    pub fn stat(&self, path: &str) -> Result<FileStat, FsError> {
        let path = self.resolve_path(path);
        let d = self.dentries.get(path).ok_or(FsError::NotFound)?;
        let ino = self.inodes.get(&d.inode).ok_or(FsError::NotFound)?;
        let owner = self
            .ownership
            .owner(path)
            .unwrap_or(Owner { uid: 0, gid: 0 });
        let mode = self.modes.get(path).copied().unwrap_or(Mode {
            owner_read: true,
            owner_write: true,
            other_read: true,
            other_write: false,
        });
        let nlink = self.inode_ops.nlink(ino.ino).unwrap_or(0);
        Ok(FileStat {
            ino: ino.ino,
            size: ino.size,
            owner,
            mode,
            nlink,
        })
    }

    pub fn mount(&mut self, path: &str, fs_type: &str) -> Result<(), FsError> {
        if !self.directories.contains(path) {
            return Err(FsError::NotFound);
        }
        if self.mounts.contains_key(path) {
            return Err(FsError::MountPointBusy);
        }
        self.mounts
            .insert(path.to_string(), MountPoint::new(path, fs_type));
        self.journal.record("mount", path);
        Ok(())
    }

    pub fn is_mounted(&self, path: &str) -> bool {
        self.mounts.contains_key(path)
    }

    pub fn set_quota_limit(&mut self, bytes: u64) {
        self.quota.set_limit(bytes);
    }

    pub fn set_inode_quota_limit(&mut self, inodes: u64) {
        self.quota.set_inode_limit(inodes);
    }

    pub fn unmount(&mut self, path: &str) -> Result<(), FsError> {
        if self.mounts.remove(path).is_some() {
            self.journal.record("unmount", path);
            Ok(())
        } else {
            Err(FsError::NotFound)
        }
    }

    pub fn list_dir(&self, path: &str) -> Result<Vec<String>, FsError> {
        if !self.directories.contains(path) {
            return Err(FsError::NotADirectory);
        }
        let prefix = if path == "/" {
            "/".to_string()
        } else {
            format!("{path}/")
        };
        let mut out = BTreeSet::new();
        for p in self
            .directories
            .iter()
            .chain(self.files.keys())
            .chain(self.symlinks.keys())
        {
            if p.starts_with(&prefix) {
                let tail = &p[prefix.len()..];
                if !tail.is_empty() && !tail.contains('/') {
                    out.insert(tail.to_string());
                }
            }
        }
        Ok(out.into_iter().collect())
    }

    pub fn rename(&mut self, old: &str, new: &str) -> Result<(), FsError> {
        Self::validate_abs_path(new)?;
        if self.files.contains_key(new)
            || self.directories.contains(new)
            || self.symlinks.contains_key(new)
        {
            return Err(FsError::AlreadyExists);
        }
        if let Some(data) = self.files.remove(old) {
            let d = self.dentries.remove(old).ok_or(FsError::NotFound)?;
            self.files.insert(new.to_string(), data);
            self.dentries
                .insert(new.to_string(), Dentry::new(new, d.inode));
            if let Some(mode) = self.modes.remove(old) {
                self.modes.insert(new.to_string(), mode);
            }
            if let Some(owner) = self.ownership.owner(old) {
                self.ownership.chown(new, owner.uid, owner.gid);
            }
            self.journal.record("rename", new);
            return Ok(());
        }
        if let Some(target) = self.symlinks.remove(old) {
            self.symlinks.insert(new.to_string(), target);
            self.journal.record("rename_symlink", new);
            return Ok(());
        }
        Err(FsError::NotFound)
    }

    pub fn rename_replace(&mut self, old: &str, new: &str) -> Result<(), FsError> {
        Self::validate_abs_path(new)?;
        if self.directories.contains(new) {
            return Err(FsError::IsDirectory);
        }
        if self.files.contains_key(new) || self.symlinks.contains_key(new) {
            let _ = self.unlink(new);
        }
        self.rename(old, new)
    }

    pub fn symlink(&mut self, link_path: &str, target_path: &str) -> Result<(), FsError> {
        Self::validate_abs_path(link_path)?;
        if self.files.contains_key(link_path)
            || self.directories.contains(link_path)
            || self.symlinks.contains_key(link_path)
        {
            return Err(FsError::AlreadyExists);
        }
        self.symlinks
            .insert(link_path.to_string(), target_path.to_string());
        self.journal.record("symlink", link_path);
        Ok(())
    }

    pub fn link(&mut self, old: &str, new: &str) -> Result<(), FsError> {
        if self.files.contains_key(new)
            || self.directories.contains(new)
            || self.symlinks.contains_key(new)
        {
            return Err(FsError::AlreadyExists);
        }
        let data = self.files.get(old).cloned().ok_or(FsError::NotFound)?;
        let d = self.dentries.get(old).cloned().ok_or(FsError::NotFound)?;
        self.files.insert(new.to_string(), data);
        self.dentries
            .insert(new.to_string(), Dentry::new(new, d.inode));
        let _ = self.inode_ops.link(d.inode);
        if let Some(mode) = self.modes.get(old).copied() {
            self.modes.insert(new.to_string(), mode);
        }
        if let Some(owner) = self.ownership.owner(old) {
            self.ownership.chown(new, owner.uid, owner.gid);
        }
        self.journal.record("link", new);
        Ok(())
    }

    pub fn open(&mut self, path: &str) -> Result<i64, FsError> {
        let path = self.resolve_path(path).to_string();
        if !self.files.contains_key(&path) {
            return Err(FsError::NotFound);
        }
        self.next_fd += 1;
        let fd = self.next_fd;
        self.open_files.insert(fd, File::new(fd, &path));
        self.journal.record("open", &path);
        Ok(fd)
    }

    pub fn close(&mut self, fd: i64) -> Result<(), FsError> {
        if let Some(f) = self.open_files.remove(&fd) {
            self.journal.record("close", &f.path);
            Ok(())
        } else {
            Err(FsError::NotFound)
        }
    }

    pub fn lseek(&mut self, fd: i64, off: usize) -> Result<(), FsError> {
        let f = self.open_files.get_mut(&fd).ok_or(FsError::NotFound)?;
        f.seek(off);
        Ok(())
    }

    pub fn read_fd(&mut self, fd: i64, len: usize) -> Result<Vec<u8>, FsError> {
        let (path, off) = {
            let f = self.open_files.get(&fd).ok_or(FsError::NotFound)?;
            (f.path.clone(), f.offset)
        };
        let data = self.read_file(&path)?;
        let end = off.saturating_add(len).min(data.len());
        let out = if off < end {
            data[off..end].to_vec()
        } else {
            Vec::new()
        };
        if let Some(f) = self.open_files.get_mut(&fd) {
            f.offset = end;
        }
        Ok(out)
    }

    pub fn write_fd(&mut self, fd: i64, bytes: &[u8]) -> Result<usize, FsError> {
        let (path, off) = {
            let f = self.open_files.get(&fd).ok_or(FsError::NotFound)?;
            (f.path.clone(), f.offset)
        };
        let mut data = self.read_file(&path)?;
        if off > data.len() {
            data.resize(off, 0);
        }
        let needed = off + bytes.len();
        if needed > data.len() {
            data.resize(needed, 0);
        }
        data[off..off + bytes.len()].copy_from_slice(bytes);
        self.write_file(&path, &data)?;
        if let Some(f) = self.open_files.get_mut(&fd) {
            f.offset = off + bytes.len();
        }
        Ok(bytes.len())
    }

    pub fn journal_entries(&self) -> &[JournalEntry] {
        self.journal.entries()
    }

    pub fn snapshot(&self) -> VfsSnapshot {
        VfsSnapshot {
            files: self.files.clone(),
            directories: self.directories.clone(),
            symlinks: self.symlinks.clone(),
        }
    }

    pub fn restore_snapshot(&mut self, snap: &VfsSnapshot) {
        self.files = snap.files.clone();
        self.directories = snap.directories.clone();
        self.symlinks = snap.symlinks.clone();
        self.page_cache = PageCache::default();
        for (path, data) in &self.files {
            if let Some(d) = self.dentries.get(path) {
                self.page_cache.put(d.inode, data);
            }
        }
    }

    pub fn replay_journal_subset(&mut self, entries: &[JournalEntry]) {
        for e in entries {
            match e.op.as_str() {
                "mkdir" => {
                    let _ = self.mkdir(&e.path);
                }
                "unlink" => {
                    let _ = self.unlink(&e.path);
                }
                "rmdir" => {
                    let _ = self.rmdir(&e.path);
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_read_write_file() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/etc").expect("mkdir failed");
        vfs.create_file("/etc/hostname", b"rust-linux")
            .expect("create failed");
        assert_eq!(
            vfs.read_file("/etc/hostname").expect("read failed"),
            b"rust-linux"
        );
        vfs.write_file("/etc/hostname", b"kernel-rs")
            .expect("write failed");
        assert_eq!(
            vfs.read_file("/etc/hostname").expect("read failed"),
            b"kernel-rs"
        );
    }

    #[test]
    fn mount_directory() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/mnt").expect("mkdir failed");
        vfs.mount("/mnt", "tmpfs").expect("mount failed");
        assert!(vfs.is_mounted("/mnt"));
    }

    #[test]
    fn rejects_invalid_parent() {
        let mut vfs = Vfs::new();
        assert_eq!(vfs.create_file("/missing/x", b"1"), Err(FsError::NotFound));
    }

    #[test]
    fn perm_ownership_and_stat() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/home").expect("mkdir");
        vfs.create_file_as("/home/a", b"abc", Cred { uid: 0 }, 1000, 1000)
            .expect("create");
        vfs.chmod(
            "/home/a",
            Mode {
                owner_read: true,
                owner_write: true,
                other_read: false,
                other_write: false,
            },
        )
        .expect("chmod");
        let st = vfs.stat("/home/a").expect("stat");
        assert_eq!(st.owner.uid, 1000);
        assert_eq!(st.size, 3);
        assert_eq!(
            vfs.read_file_as("/home/a", Cred { uid: 2000 }),
            Err(FsError::PermissionDenied)
        );
    }

    #[test]
    fn quota_blocks_large_create() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/q").expect("mkdir");
        vfs.set_quota_limit(4);
        assert_eq!(
            vfs.create_file("/q/big", b"12345"),
            Err(FsError::QuotaExceeded)
        );
    }

    #[test]
    fn unlink_removes_file() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/tmp").expect("mkdir");
        vfs.create_file("/tmp/x", b"1").expect("create");
        vfs.unlink("/tmp/x").expect("unlink");
        assert_eq!(vfs.read_file("/tmp/x"), Err(FsError::NotFound));
    }

    #[test]
    fn rename_and_listdir_flow() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/a").expect("mkdir");
        vfs.create_file("/a/f1", b"x").expect("create");
        vfs.rename("/a/f1", "/a/f2").expect("rename");
        assert_eq!(vfs.read_file("/a/f2").expect("read"), b"x");
        let ls = vfs.list_dir("/a").expect("list");
        assert_eq!(ls, vec!["f2".to_string()]);
    }

    #[test]
    fn symlink_and_hardlink_flow() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/b").expect("mkdir");
        vfs.create_file("/b/orig", b"hello").expect("create");
        vfs.symlink("/b/sym", "/b/orig").expect("symlink");
        assert_eq!(vfs.read_file("/b/sym").expect("read"), b"hello");
        vfs.link("/b/orig", "/b/hard").expect("hardlink");
        assert_eq!(vfs.read_file("/b/hard").expect("read"), b"hello");
        let st = vfs.stat("/b/orig").expect("stat");
        assert!(st.nlink >= 2);
    }

    #[test]
    fn mount_and_unmount_flow() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/mnt").expect("mkdir");
        vfs.mount("/mnt", "tmpfs").expect("mount");
        assert!(vfs.is_mounted("/mnt"));
        vfs.unmount("/mnt").expect("unmount");
        assert!(!vfs.is_mounted("/mnt"));
    }

    #[test]
    fn fd_read_write_seek_flow() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/fd").expect("mkdir");
        vfs.create_file("/fd/a", b"abcdef").expect("create");
        let fd = vfs.open("/fd/a").expect("open");
        assert_eq!(vfs.read_fd(fd, 3).expect("read"), b"abc");
        vfs.lseek(fd, 2).expect("seek");
        assert_eq!(vfs.write_fd(fd, b"ZZ").expect("write"), 2);
        vfs.close(fd).expect("close");
        assert_eq!(vfs.read_file("/fd/a").expect("read"), b"abZZef");
    }

    #[test]
    fn journal_captures_ops() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/j").expect("mkdir");
        vfs.create_file("/j/x", b"1").expect("create");
        let fd = vfs.open("/j/x").expect("open");
        let _ = vfs.write_fd(fd, b"2").expect("write");
        vfs.close(fd).expect("close");
        vfs.unlink("/j/x").expect("unlink");
        assert!(vfs.journal_entries().len() >= 5);
    }

    #[test]
    fn rename_replace_overwrites_destination() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/r").expect("mkdir");
        vfs.create_file("/r/a", b"A").expect("a");
        vfs.create_file("/r/b", b"B").expect("b");
        vfs.rename_replace("/r/a", "/r/b").expect("rename_replace");
        assert_eq!(vfs.read_file("/r/b").expect("read"), b"A");
        assert_eq!(vfs.read_file("/r/a"), Err(FsError::NotFound));
    }

    #[test]
    fn snapshot_restore_roundtrip() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/s").expect("mkdir");
        vfs.create_file("/s/x", b"1").expect("create");
        let snap = vfs.snapshot();
        vfs.write_file("/s/x", b"2").expect("write");
        assert_eq!(vfs.read_file("/s/x").expect("read"), b"2");
        vfs.restore_snapshot(&snap);
        assert_eq!(vfs.read_file("/s/x").expect("read"), b"1");
    }

    #[test]
    fn replay_subset_applies_ops() {
        let mut src = Vfs::new();
        src.mkdir("/j2").expect("mkdir");
        src.create_file("/j2/f", b"x").expect("create");
        src.unlink("/j2/f").expect("unlink");
        let entries = src.journal_entries().to_vec();

        let mut dst = Vfs::new();
        dst.replay_journal_subset(&entries);
        assert!(dst.directories.contains("/j2"));
        assert_eq!(dst.read_file("/j2/f"), Err(FsError::NotFound));
    }

    #[test]
    fn stress_many_files_consistent() {
        let mut vfs = Vfs::new();
        vfs.mkdir("/stress").expect("mkdir");
        for i in 0..128 {
            let p = format!("/stress/f{i}");
            vfs.create_file(&p, b"x").expect("create");
        }
        let ls = vfs.list_dir("/stress").expect("list");
        assert_eq!(ls.len(), 128);
        for i in 0..64 {
            let p = format!("/stress/f{i}");
            vfs.unlink(&p).expect("unlink");
        }
        let ls2 = vfs.list_dir("/stress").expect("list2");
        assert_eq!(ls2.len(), 64);
    }
}
