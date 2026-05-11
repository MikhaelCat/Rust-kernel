use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct Efi {
    pub runtime: bool,
    vars: BTreeMap<String, String>,
}

impl Efi {
    pub fn enable_runtime(&mut self) {
        self.runtime = true;
    }

    pub fn set_var(&mut self, key: &str, val: &str) {
        self.vars.insert(key.to_string(), val.to_string());
    }

    pub fn get_var(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn efi_runtime_and_vars() {
        let mut e = Efi::default();
        e.enable_runtime();
        e.set_var("BootOrder", "0001,0002");
        assert!(e.runtime);
        assert_eq!(e.get_var("BootOrder"), Some("0001,0002"));
    }
}
