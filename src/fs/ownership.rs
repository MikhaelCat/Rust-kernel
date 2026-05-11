use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Owner {
    pub uid: u32,
    pub gid: u32,
}

#[derive(Debug, Default)]
pub struct OwnershipTable {
    owners: BTreeMap<String, Owner>,
}

impl OwnershipTable {
    pub fn chown(&mut self, path: &str, uid: u32, gid: u32) {
        self.owners.insert(path.to_string(), Owner { uid, gid });
    }

    pub fn owner(&self, path: &str) -> Option<Owner> {
        self.owners.get(path).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chown_lookup() {
        let mut t = OwnershipTable::default();
        t.chown("/etc/hosts", 0, 0);
        assert_eq!(t.owner("/etc/hosts"), Some(Owner { uid: 0, gid: 0 }));
    }
}
