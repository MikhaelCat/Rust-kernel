use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    AlreadyRegistered,
    NotFound,
}

#[derive(Debug, Default)]
pub struct DriverRegistry {
    drivers: BTreeSet<String>,
}

impl DriverRegistry {
    pub fn new() -> Self {
        Self {
            drivers: BTreeSet::new(),
        }
    }

    pub fn register(&mut self, name: &str) -> Result<(), DriverError> {
        if !self.drivers.insert(name.to_string()) {
            return Err(DriverError::AlreadyRegistered);
        }
        Ok(())
    }

    pub fn unregister(&mut self, name: &str) -> Result<(), DriverError> {
        if !self.drivers.remove(name) {
            return Err(DriverError::NotFound);
        }
        Ok(())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.drivers.contains(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_unregister_driver() {
        let mut reg = DriverRegistry::new();
        reg.register("e1000").expect("register failed");
        assert!(reg.contains("e1000"));
        reg.unregister("e1000").expect("unregister failed");
        assert!(!reg.contains("e1000"));
    }
}
