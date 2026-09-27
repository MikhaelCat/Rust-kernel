//! Extended File System Operations (Ext4-style)

use super::vfs::{Dentry, Inode, Vfs};

#[derive(Debug, Clone)]
pub struct Ext4Inode {
    pub base: Inode,
    pub flags: u32,
    pub links: u32,
    pub block_map: Vec<BlockMapEntry>,
}

#[derive(Debug, Clone)]
pub struct BlockMapEntry {
    pub logical_block: u64,
    pub physical_block: u64,
    pub blocks: u32,
}

impl Default for Ext4Inode {
    fn default() -> Self {
        Self {
            base: Inode::default(),
            flags: 0,
            links: 1,
            block_map: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Ext4Filesystem {
    pub fs_root: Dentry,
    pub inodes: Vec<Ext4Inode>,
    pub next_inode: u64,
    pub blocks_total: u64,
    pub blocks_used: u64,
}

impl Default for Ext4Filesystem {
    fn default() -> Self {
        Self::new()
    }
}

impl Ext4Filesystem {
    pub fn new() -> Self {
        let mut root = Dentry::new("/");
        root.inode.permissions = 0o755;
        
        Self {
            fs_root: root,
            inodes: Vec::new(),
            next_inode: 1,
            blocks_total: 1024 * 1024, // 4GB
            blocks_used: 0,
        }
    }

    pub fn allocate_inodes(&mut self) -> u64 {
        let id = self.next_inode;
        self.inodes.push(Ext4Inode::default());
        self.next_inode += 1;
        id
    }

    pub fn extend_file(&mut self, inode_id: u64, new_blocks: u64) -> Result<(), &'static str> {
        if inode_id >= self.inodes.len() as u64 {
            return Err("Invalid inode");
        }
        self.blocks_used += new_blocks;
        Ok(())
    }
}
