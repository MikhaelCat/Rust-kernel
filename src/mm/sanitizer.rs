#[derive(Debug, Default)]
pub struct Sanitizer {
    checks: u64,
}

impl Sanitizer {
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
    fn records_checks() {
        let mut s = Sanitizer::default();
        s.check();
        assert_eq!(s.checks(), 1);
    }
}
