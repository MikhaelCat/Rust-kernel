#[derive(Debug, Default)]
pub struct GpioBank {
    pins_high: u32,
}
impl GpioBank {
    pub fn set_high(&mut self) {
        self.pins_high += 1;
    }
    pub fn pins_high(&self) -> u32 {
        self.pins_high
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gpio_set() {
        let mut g = GpioBank::default();
        g.set_high();
        assert_eq!(g.pins_high(), 1);
    }
}
