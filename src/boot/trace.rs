#[derive(Debug, Default)]
pub struct BootTrace {
    marks: u64,
}
impl BootTrace {
    pub fn mark(&mut self) {
        self.marks += 1;
    }
    pub fn marks(&self) -> u64 {
        self.marks
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mark_trace() {
        let mut t = BootTrace::default();
        t.mark();
        assert_eq!(t.marks(), 1);
    }
}
