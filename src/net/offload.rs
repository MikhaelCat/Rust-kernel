#[derive(Debug, Default)]
pub struct Offload {
    tso: bool,
    gro: bool,
}

impl Offload {
    pub fn enable_tso(&mut self) {
        self.tso = true;
    }
    pub fn enable_gro(&mut self) {
        self.gro = true;
    }
    pub fn enabled(&self) -> bool {
        self.tso || self.gro
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enable_features() {
        let mut o = Offload::default();
        o.enable_tso();
        assert!(o.enabled());
    }
}
