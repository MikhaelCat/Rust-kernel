#[derive(Debug, Default)]
pub struct Quota {
    pub used: u64,
    pub limit: u64,
    pub inodes_used: u64,
    pub inodes_limit: u64,
}
impl Quota {
    pub fn set_limit(&mut self, lim: u64) {
        self.limit = lim;
    }
    pub fn reserve(&mut self, bytes: u64) -> bool {
        if self.used + bytes <= self.limit {
            self.used += bytes;
            true
        } else {
            false
        }
    }

    pub fn set_inode_limit(&mut self, lim: u64) {
        self.inodes_limit = lim;
    }

    pub fn reserve_inode(&mut self) -> bool {
        if self.inodes_limit == 0 {
            return true;
        }
        if self.inodes_used + 1 <= self.inodes_limit {
            self.inodes_used += 1;
            true
        } else {
            false
        }
    }

    pub fn release_inode(&mut self) {
        self.inodes_used = self.inodes_used.saturating_sub(1);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reserve_quota() {
        let mut q = Quota::default();
        q.set_limit(100);
        assert!(q.reserve(70));
        assert!(!q.reserve(40));
    }

    #[test]
    fn inode_quota() {
        let mut q = Quota::default();
        q.set_inode_limit(2);
        assert!(q.reserve_inode());
        assert!(q.reserve_inode());
        assert!(!q.reserve_inode());
        q.release_inode();
        assert!(q.reserve_inode());
    }
}
