#[derive(Debug, Default)]
pub struct LoadAvg {
    pub one: f32,
}
impl LoadAvg {
    pub fn update(&mut self, v: f32) {
        self.one = v;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_load() {
        let mut l = LoadAvg::default();
        l.update(0.5);
        assert!((l.one - 0.5).abs() < 0.0001);
    }
}
