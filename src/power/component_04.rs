#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerComponent04 {
    pub name: &'static str,
    pub enabled: bool,
}

impl PowerComponent04 {
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
        let mut c = PowerComponent04::new("comp");
        assert!(c.enabled);
        c.disable();
        assert!(!c.enabled);
    }
}
