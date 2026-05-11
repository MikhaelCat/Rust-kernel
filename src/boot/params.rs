use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootParams {
    pub cmdline: String,
    pub initrd_present: bool,
    pub args: BTreeMap<String, String>,
}

impl BootParams {
    pub fn new(cmdline: &str, initrd_present: bool) -> Self {
        Self {
            cmdline: cmdline.to_string(),
            initrd_present,
            args: parse_cmdline(cmdline),
        }
    }

    pub fn validate(&self) -> bool {
        !self.cmdline.trim().is_empty()
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.args.get(key).map(String::as_str)
    }
}

fn parse_cmdline(cmdline: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for token in cmdline.split_whitespace() {
        if let Some((k, v)) = token.split_once('=') {
            out.insert(k.to_string(), v.to_string());
        } else {
            out.insert(token.to_string(), String::new());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_parse_params() {
        let p = BootParams::new("console=ttyS0 quiet", true);
        assert!(p.initrd_present);
        assert_eq!(p.get("console"), Some("ttyS0"));
        assert_eq!(p.get("quiet"), Some(""));
    }

    #[test]
    fn validate_cmdline() {
        assert!(BootParams::new("root=/dev/ram0", false).validate());
        assert!(!BootParams::new("   ", false).validate());
    }
}
