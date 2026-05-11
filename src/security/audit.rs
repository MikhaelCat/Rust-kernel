#[derive(Debug, Default)]
pub struct AuditLog {
    events: Vec<String>,
}
impl AuditLog {
    pub fn record(&mut self, e: &str) {
        self.events.push(e.to_string());
    }
    pub fn count(&self) -> usize {
        self.events.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn records() {
        let mut a = AuditLog::default();
        a.record("mount");
        assert_eq!(a.count(), 1);
    }
}
