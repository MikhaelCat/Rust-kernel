#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitCmdline {
    pub raw: String,
}
impl InitCmdline {
    pub fn new(raw: &str) -> Self {
        Self {
            raw: raw.to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_cmdline() {
        assert_eq!(InitCmdline::new("a").raw, "a");
    }
}
