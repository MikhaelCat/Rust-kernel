use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct InodeOps {
    links: BTreeMap<u64, u32>,
}

impl InodeOps {
    pub fn create_inode(&mut self, ino: u64) {
        self.links.entry(ino).or_insert(1);
    }

    pub fn link(&mut self, ino: u64) -> bool {
        if let Some(n) = self.links.get_mut(&ino) {
            *n += 1;
            true
        } else {
            false
        }
    }

    pub fn unlink(&mut self, ino: u64) -> bool {
        if let Some(n) = self.links.get_mut(&ino) {
            if *n > 1 {
                *n -= 1;
            } else {
                self.links.remove(&ino);
            }
            true
        } else {
            false
        }
    }

    pub fn nlink(&self, ino: u64) -> Option<u32> {
        self.links.get(&ino).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_unlink_flow() {
        let mut ops = InodeOps::default();
        ops.create_inode(1);
        assert_eq!(ops.nlink(1), Some(1));
        assert!(ops.link(1));
        assert_eq!(ops.nlink(1), Some(2));
        assert!(ops.unlink(1));
        assert_eq!(ops.nlink(1), Some(1));
        assert!(ops.unlink(1));
        assert_eq!(ops.nlink(1), None);
    }
}
