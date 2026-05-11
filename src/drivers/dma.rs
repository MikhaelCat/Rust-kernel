#[derive(Debug, Default)]
pub struct DmaEngine {
    submitted: u64,
}
impl DmaEngine {
    pub fn submit(&mut self) {
        self.submitted += 1;
    }
    pub fn submitted(&self) -> u64 {
        self.submitted
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dma_submit() {
        let mut d = DmaEngine::default();
        d.submit();
        assert_eq!(d.submitted(), 1);
    }
}
