use std::collections::BTreeSet;
#[derive(Debug, Default)]
pub struct Dcache {
    names: BTreeSet<String>,
}
impl Dcache {
    pub fn insert(&mut self, n: &str) {
        self.names.insert(n.to_string());
    }
    pub fn contains(&self, n: &str) -> bool {
        self.names.contains(n)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn insert_dentry() {
        let mut d = Dcache::default();
        d.insert("/etc");
        assert!(d.contains("/etc"));
    }
}
