#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeComponent03 {
    pub name: &'static str,
    pub enabled: bool,
}

impl TimeComponent03 {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_lifecycle() {
        let mut c = TimeComponent03::new("comp");
        assert!(c.enabled);
        c.disable();
        assert!(!c.enabled);
    }
}
