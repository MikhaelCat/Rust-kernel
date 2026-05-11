#[derive(Debug, Default)]
pub struct PanicLog {
    lines: Vec<String>,
}
impl PanicLog {
    pub fn push(&mut self, s: &str) {
        self.lines.push(s.to_string());
    }
    pub fn len(&self) -> usize {
        self.lines.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_line() {
        let mut p = PanicLog::default();
        p.push("oops");
        assert_eq!(p.len(), 1);
    }
}
