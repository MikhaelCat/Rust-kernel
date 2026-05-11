use std::collections::BTreeMap;
#[derive(Debug, Default)]
pub struct Kallsyms {
    sym: BTreeMap<String, u64>,
}
impl Kallsyms {
    pub fn add(&mut self, n: &str, a: u64) {
        self.sym.insert(n.to_string(), a);
    }
    pub fn addr(&self, n: &str) -> Option<u64> {
        self.sym.get(n).copied()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_lookup() {
        let mut k = Kallsyms::default();
        k.add("start", 0x1000);
        assert_eq!(k.addr("start"), Some(0x1000));
    }
}
