//! USB Device Types

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub device_descriptor: DeviceDescriptor,
    pub config_descriptors: Vec<ConfigDescriptor>,
    pub interface_descriptors: Vec<InterfaceDescriptor>,
    pub endpoint_descriptors: Vec<EndpointDescriptor>,
    pub state: DeviceState,
    pub address: u8,
    pub speed: Speed,
}

#[derive(Debug, Clone)]
pub struct DeviceDescriptor {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bcdUSB: u16,
    pub bDeviceClass: u8,
    pub bDeviceSubClass: u8,
    pub bDeviceProtocol: u8,
    pub bMaxPacketSize: u8,
    pub idVendor: u16,
    pub idProduct: u16,
    pub bcdDevice: u16,
    pub iManufacturer: u8,
    pub iProduct: u8,
    pub iSerialNumber: u8,
    pub bNumConfigurations: u8,
}

#[derive(Debug, Clone)]
pub struct ConfigDescriptor {
    pub wTotalLength: u16,
    pub bNumInterfaces: u8,
    pub bConfigurationValue: u8,
    pub iConfiguration: u8,
    pub bmAttributes: u8,
    pub maxPower: u8,
}

#[derive(Debug, Clone)]
pub struct InterfaceDescriptor {
    pub bInterfaceNumber: u8,
    pub bAlternateSetting: u8,
    pub bNumEndpoints: u8,
    pub bInterfaceClass: u8,
    pub bInterfaceSubClass: u8,
    pub bInterfaceProtocol: u8,
    pub iInterface: u8,
}

#[derive(Debug, Clone)]
pub struct EndpointDescriptor {
    pub bEndpointAddress: u8,
    pub bmAttributes: u8,
    pub wMaxPacketSize: u16,
    pub bInterval: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceState {
    Detached,
    Attached,
    Configured,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Speed {
    Low,
    Full,
    High,
    Super,
}
