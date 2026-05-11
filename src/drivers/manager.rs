use super::error::DriverCoreError;
use super::registry::DriverRegistry;

#[derive(Debug, Default)]
pub struct DriverManager {
    reg: DriverRegistry,
}

impl DriverManager {
    pub fn register(&mut self, name: &str) {
        let _ = self.reg.register(name);
    }

    pub fn unregister(&mut self, name: &str) -> Result<(), DriverCoreError> {
        self.reg
            .unregister(name)
            .map_err(|_| DriverCoreError::NotRegistered)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.reg.contains(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_unregister() {
        let mut d = DriverManager::default();
        d.register("e1000");
        assert!(d.contains("e1000"));
        d.unregister("e1000").expect("unregister failed");
        assert!(!d.contains("e1000"));
    }
}
