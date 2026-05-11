#[derive(Debug, Default)]
pub struct RtStats {
    pub budget: i64,
}
impl RtStats {
    pub fn consume(&mut self) {
        self.budget -= 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rt_consume() {
        let mut s = RtStats { budget: 3 };
        s.consume();
        assert_eq!(s.budget, 2);
    }
}
