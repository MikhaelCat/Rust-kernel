use std::collections::BTreeMap;
#[derive(Debug, Default)]
pub struct NeighTable {
    m: BTreeMap<[u8; 16], [u8; 6]>,
}
impl NeighTable {
    pub fn insert(&mut self, ip: [u8; 16], mac: [u8; 6]) {
        self.m.insert(ip, mac);
    }
    pub fn len(&self) -> usize {
        self.m.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn neigh_insert() {
        let mut n = NeighTable::default();
        n.insert([0; 16], [1, 2, 3, 4, 5, 6]);
        assert_eq!(n.len(), 1);
    }
}
