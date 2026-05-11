#[derive(Debug, Default)]
pub struct PowerDomain {
    enabled: bool,
}

impl PowerDomain {
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_enable() {
        let mut p = PowerDomain::default();
        p.enable();
        assert!(p.enabled());
    }
}
