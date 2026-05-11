use super::{DriverManager, DriverRegistry};

#[derive(Debug, Default)]
pub struct DriverSystem {
    manager: DriverManager,
    _compat: DriverRegistry,
}

impl DriverSystem {
    pub fn load_base(&mut self) {
        self.manager.register("uart");
        self.manager.register("net");
    }

    pub fn has(&self, name: &str) -> bool {
        self.manager.contains(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_base_drivers() {
        let mut d = DriverSystem::default();
        d.load_base();
        assert!(d.has("uart"));
        assert!(d.has("net"));
    }
}
