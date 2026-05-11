#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InodeKind {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inode {
    pub ino: u64,
    pub kind: InodeKind,
    pub size: u64,
}

impl Inode {
    pub fn new(ino: u64, kind: InodeKind) -> Self {
        Self { ino, kind, size: 0 }
    }
}
