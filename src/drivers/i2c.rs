#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I2cDevice {
    pub addr: u16,
}
impl I2cDevice {
    pub fn new(addr: u16) -> Self {
        Self { addr }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn i2c_new() {
        assert_eq!(I2cDevice::new(0x50).addr, 0x50);
    }
}
