#[derive(Debug, Default)]
pub struct Kasan {
    enabled: bool,
}
impl Kasan {
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
    fn kasan_enable() {
        let mut k = Kasan::default();
        k.enable();
        assert!(k.enabled());
    }
}
