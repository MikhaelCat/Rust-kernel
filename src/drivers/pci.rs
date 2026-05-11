#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PciDevice {
    pub vendor: u16,
    pub device: u16,
}
impl PciDevice {
    pub fn new(vendor: u16, device: u16) -> Self {
        Self { vendor, device }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_pci() {
        assert_eq!(PciDevice::new(0x8086, 0x100e).vendor, 0x8086);
    }
}
