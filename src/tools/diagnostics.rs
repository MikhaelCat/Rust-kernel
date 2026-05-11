#[derive(Debug, Default)]
pub struct Diagnostics {
    checks: u64,
}

impl Diagnostics {
    pub fn check(&mut self) {
        self.checks += 1;
    }

    pub fn checks(&self) -> u64 {
        self.checks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_count() {
        let mut d = Diagnostics::default();
        d.check();
        assert_eq!(d.checks(), 1);
    }
}
