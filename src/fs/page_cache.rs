use std::collections::{BTreeMap, BTreeSet};
#[derive(Debug, Default)]
pub struct PageCache {
    hot: BTreeSet<u64>,
    pages: BTreeMap<u64, Vec<u8>>,
}
impl PageCache {
    pub fn mark_hot(&mut self, ino: u64) {
        self.hot.insert(ino);
    }
    pub fn is_hot(&self, ino: u64) -> bool {
        self.hot.contains(&ino)
    }

    pub fn put(&mut self, ino: u64, data: &[u8]) {
        self.pages.insert(ino, data.to_vec());
        self.mark_hot(ino);
    }

    pub fn get(&self, ino: u64) -> Option<&[u8]> {
        self.pages.get(&ino).map(Vec::as_slice)
    }

    pub fn drop_inode(&mut self, ino: u64) {
        self.pages.remove(&ino);
        self.hot.remove(&ino);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cache_hot() {
        let mut c = PageCache::default();
        c.mark_hot(3);
        assert!(c.is_hot(3));
    }

    #[test]
    fn cache_put_get_drop() {
        let mut c = PageCache::default();
        c.put(7, b"abc");
        assert_eq!(c.get(7), Some(&b"abc"[..]));
        c.drop_inode(7);
        assert_eq!(c.get(7), None);
    }
}
