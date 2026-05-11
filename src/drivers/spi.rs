#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpiDevice {
    pub cs: u8,
}
impl SpiDevice {
    pub fn new(cs: u8) -> Self {
        Self { cs }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spi_new() {
        assert_eq!(SpiDevice::new(1).cs, 1);
    }
}
