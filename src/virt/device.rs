#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtDevice {
    Net { name: String },
    Block { name: String },
    Console,
}

#[derive(Debug, Default)]
pub struct DeviceSet {
    devices: Vec<VirtDevice>,
}

impl DeviceSet {
    pub fn add(&mut self, d: VirtDevice) {
        self.devices.push(d);
    }

    pub fn count(&self) -> usize {
        self.devices.len()
    }

    pub fn has_net(&self) -> bool {
        self.devices
            .iter()
            .any(|d| matches!(d, VirtDevice::Net { .. }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_devices() {
        let mut ds = DeviceSet::default();
        ds.add(VirtDevice::Console);
        ds.add(VirtDevice::Net {
            name: "virtio-net0".into(),
        });
        assert_eq!(ds.count(), 2);
        assert!(ds.has_net());
    }
}
