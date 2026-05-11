use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlabObject {
    pub id: u64,
    pub size: usize,
}

#[derive(Debug, Default)]
pub struct SlabCache {
    pub objects: usize,
    next_id: u64,
    live: BTreeMap<u64, SlabObject>,
    bytes_in_use: usize,
}

impl SlabCache {
    pub fn alloc(&mut self) {
        let _ = self.kmalloc(64);
    }

    pub fn kmalloc(&mut self, size: usize) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        let obj = SlabObject { id, size };
        self.live.insert(id, obj);
        self.objects = self.live.len();
        self.bytes_in_use += size;
        id
    }

    pub fn kfree(&mut self, id: u64) -> bool {
        if let Some(obj) = self.live.remove(&id) {
            self.objects = self.live.len();
            self.bytes_in_use = self.bytes_in_use.saturating_sub(obj.size);
            true
        } else {
            false
        }
    }

    pub fn bytes_in_use(&self) -> usize {
        self.bytes_in_use
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alloc_obj() {
        let mut c = SlabCache::default();
        c.alloc();
        assert_eq!(c.objects, 1);
    }

    #[test]
    fn kmalloc_kfree_flow() {
        let mut c = SlabCache::default();
        let a = c.kmalloc(128);
        let b = c.kmalloc(64);
        assert_eq!(c.objects, 2);
        assert_eq!(c.bytes_in_use(), 192);
        assert!(c.kfree(a));
        assert!(c.kfree(b));
        assert_eq!(c.bytes_in_use(), 0);
    }
}
