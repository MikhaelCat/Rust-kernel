#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dentry {
    pub name: String,
    pub inode: u64,
}

impl Dentry {
    pub fn new(name: &str, inode: u64) -> Self {
        Self {
            name: name.to_string(),
            inode,
        }
    }
}
