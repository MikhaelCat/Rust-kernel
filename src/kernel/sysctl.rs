use std::collections::BTreeMap;
#[derive(Debug, Default)]
pub struct Sysctl {
    kv: BTreeMap<String, String>,
}
impl Sysctl {
    pub fn set(&mut self, k: &str, v: &str) {
        self.kv.insert(k.to_string(), v.to_string());
    }
    pub fn get(&self, k: &str) -> Option<&str> {
        self.kv.get(k).map(String::as_str)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sysctl_set_get() {
        let mut s = Sysctl::default();
        s.set("kernel.printk", "4");
        assert_eq!(s.get("kernel.printk"), Some("4"));
    }
}
