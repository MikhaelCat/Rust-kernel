#[derive(Debug, Default)]
pub struct DeviceBus {
    devices: usize,
}
impl DeviceBus {
    pub fn attach(&mut self) {
        self.devices += 1;
    }
    pub fn count(&self) -> usize {
        self.devices
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn attach_device() {
        let mut b = DeviceBus::default();
        b.attach();
        assert_eq!(b.count(), 1);
    }
}
