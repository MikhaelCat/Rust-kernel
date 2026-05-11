#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuContext {
    pub rip: u64,
    pub rsp: u64,
}
impl CpuContext {
    pub fn new() -> Self {
        Self { rip: 0, rsp: 0 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn context_zeroed() {
        let c = CpuContext::new();
        assert_eq!(c.rip, 0);
    }
}
