#[derive(Debug, Default)]
pub struct BootLog {
    entries: Vec<&'static str>,
}
impl BootLog {
    pub fn push(&mut self, e: &'static str) {
        self.entries.push(e);
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn log_push() {
        let mut l = BootLog::default();
        l.push("x");
        assert_eq!(l.len(), 1);
    }
}
