#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpcComponent01 {
    pub name: &'static str,
    pub enabled: bool,
}

impl IpcComponent01 {
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
        let mut c = IpcComponent01::new("comp");
        assert!(c.enabled);
        c.disable();
        assert!(!c.enabled);
    }
}
