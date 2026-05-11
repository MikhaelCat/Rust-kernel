use std::collections::BTreeSet;
#[derive(Debug, Default)]
pub struct Keyring {
    keys: BTreeSet<String>,
}
impl Keyring {
    pub fn add(&mut self, k: &str) {
        self.keys.insert(k.to_string());
    }
    pub fn contains(&self, k: &str) -> bool {
        self.keys.contains(k)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyring_add() {
        let mut k = Keyring::default();
        k.add("k1");
        assert!(k.contains("k1"));
    }
}
