//! PCI device types
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u32,
    pub initialized: AtomicBool,
    pub refcount: AtomicUsize,
}

impl PciDevice {
    pub fn new(vendor: u16, device: u16) -> Self {
        Self {
            vendor_id: vendor,
            device_id: device,
            class_code: 0,
            initialized: AtomicBool::new(false),
            refcount: AtomicUsize::new(1),
        }
    }
}
