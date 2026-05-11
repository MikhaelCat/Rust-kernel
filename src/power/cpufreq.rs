#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuFreq {
    pub khz: u64,
}
impl CpuFreq {
    pub fn set(khz: u64) -> Self {
        Self { khz }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_freq() {
        assert_eq!(CpuFreq::set(1800000).khz, 1800000);
    }
}
