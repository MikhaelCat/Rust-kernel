//! PCI Device Types

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u32,
    pub subclass_code: u8,
    pub programming_interface: u8,
    pub bar: [usize; 6],
    pub interrupt_pin: u8,
    pub irq: i32,
}

impl PciDevice {
    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        Self {
            bus,
            device,
            function,
            vendor_id: 0,
            device_id: 0,
            class_code: 0,
            subclass_code: 0,
            programming_interface: 0,
            bar: [0; 6],
            interrupt_pin: 0,
            irq: -1,
        }
    }
    
    pub fn address(&self) -> usize {
        (self.bus as usize) << 20 | (self.device as usize) << 15 | (self.function as usize) << 12
    }
}
