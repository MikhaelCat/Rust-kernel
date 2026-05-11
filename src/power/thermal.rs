#[derive(Debug, Default)]
pub struct Thermal {
    celsius: i32,
}
impl Thermal {
    pub fn set_temp(&mut self, t: i32) {
        self.celsius = t;
    }
    pub fn temp(&self) -> i32 {
        self.celsius
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_temp() {
        let mut t = Thermal::default();
        t.set_temp(70);
        assert_eq!(t.temp(), 70);
    }
}
