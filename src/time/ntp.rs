#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NtpAdjust {
    pub ppm: i32,
}
impl NtpAdjust {
    pub fn new(ppm: i32) -> Self {
        Self { ppm }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ntp_new() {
        assert_eq!(NtpAdjust::new(10).ppm, 10);
    }
}
