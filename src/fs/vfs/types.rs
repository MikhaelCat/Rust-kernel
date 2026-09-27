//! VFS Types

#[derive(Debug, Clone)]
pub struct Inode {
    pub inode_number: u64,
    pub mode: u16,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
    pub atime: u64,
    pub mtime: u64,
    pub ctime: u64,
    pub block_count: u64,
    pub nlink: u32,
}

impl Inode {
    pub fn new(inode_number: u64) -> Self {
        Self {
            inode_number,
            mode: 0o644,
            uid: 0,
            gid: 0,
            size: 0,
            atime: 0,
            mtime: 0,
            ctime: 0,
            block_count: 0,
            nlink: 1,
        }
    }
    
    pub fn is_file(&self) -> bool {
        self.mode & 0o170000 == 0o100000
    }
    
    pub fn is_directory(&self) -> bool {
        self.mode & 0o170000 == 0o040000
    }
    
    pub fn set_permissions(&mut self, mode: u16) {
        self.mode = mode;
    }
}

#[derive(Debug, Clone)]
pub struct Dentry {
    pub name: String,
    pub inode: u64,
    pub parent: Option<u64>,
    pub flags: DentryFlags,
}

#[derive(Debug, Clone, Copy)]
pub struct DentryFlags(pub u32);

impl DentryFlags {
    pub const NEW: DentryFlags = DentryFlags(1 << 0);
    pub const NEGATIVE: DentryFlags = DentryFlags(1 << 1);
    pub const DIRTY: DentryFlags = DentryFlags(1 << 2);
}
