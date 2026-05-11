#[derive(Debug, Default)]
pub struct Regulator {
    enabled: bool,
}
impl Regulator {
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
    fn enable_reg() {
        let mut r = Regulator::default();
        r.enable();
        assert!(r.enabled());
    }
}
