use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct MmapTable {
    next_addr: i64,
    maps: BTreeMap<i64, usize>,
}

impl MmapTable {
    pub fn new() -> Self {
        Self {
            next_addr: 0x1000_0000,
            maps: BTreeMap::new(),
        }
    }

    pub fn mmap(&mut self, len: usize) -> i64 {
        if len == 0 {
            return -22; // EINVAL
        }
        let addr = self.next_addr;
        self.next_addr += 0x1000;
        self.maps.insert(addr, len);
        addr
    }

    pub fn munmap(&mut self, addr: i64) -> i64 {
        if self.maps.remove(&addr).is_some() {
            0
        } else {
            -22
        }
    }
}
