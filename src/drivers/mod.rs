//! Device Drivers Framework Implementation
//! Реализация фреймворка драйверов устройств для ядра Linux на Rust

pub mod pci;
pub mod usb;

use std::sync::Arc;
use crate::pci::PciDevice;

// ============================================================================
// DRIVER MODEL CORE
// ============================================================================

/// Trait for all drivers
pub trait Driver: Sync + Send {
    fn name(&self) -> &str;
    fn match_device(&self, device: &dyn Any) -> bool;
    fn probe(&mut self, device: &dyn Any) -> Result<(), DriverError>;
    fn remove(&mut self, device: &dyn Any) -> Result<(), DriverError>;
}

pub trait Any: Send + Sync {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static + Send + Sync> Any for T {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// Error types for driver operations
#[derive(Debug, Clone, PartialEq)]
pub enum DriverError {
    DeviceNotFound,
    AlreadyAttached,
    ResourceAllocationFailed,
    InvalidConfiguration,
    HardwareError(i32),
    PermissionDenied,
    NotImplemented,
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::DeviceNotFound => write!(f, "Device not found"),
            DriverError::AlreadyAttached => write!(f, "Device already attached"),
            DriverError::ResourceAllocationFailed => write!(f, "Resource allocation failed"),
            DriverError::InvalidConfiguration => write!(f, "Invalid device configuration"),
            DriverError::HardwareError(code) => write!(f, "Hardware error {}", code),
            DriverError::PermissionDenied => write!(f, "Permission denied"),
            DriverError::NotImplemented => write!(f, "Not implemented"),
        }
    }
}

impl std::error::Error for DriverError {}

/// Device Manager - центральная регистрация и жизнь циклов драйверов
#[derive(Debug)]
pub struct DriverManager {
    pub drivers: HashMap<String, Box<dyn AnyDriver>>,
    pub attached_devices: HashMap<String, Vec<PciDevice>>,
}

trait AnyDriver: Send + Sync {
    fn name(&self) -> String;
    fn match_device(&self, device: &PciDevice) -> bool;
    fn probe(&mut self, device: &PciDevice) -> Result<(), DriverError>;
    fn remove(&mut self, device: &PciDevice) -> Result<(), DriverError>;
}

impl<T: Driver + 'static> AnyDriver for T {
    fn name(&self) -> String { self.name().to_string() }
    
    fn match_device(&self, device: &PciDevice) -> bool {
        // For this simplified version, we use PCI matching
        true
    }
    
    fn probe(&mut self, device: &PciDevice) -> Result<(), DriverError> {
        Ok(())
    }
    
    fn remove(&mut self, device: &PciDevice) -> Result<(), DriverError> {
        Ok(())
    }
}

impl Default for DriverManager {
    fn default() -> Self {
        Self {
            drivers: HashMap::new(),
            attached_devices: HashMap::new(),
        }
    }
}

impl DriverManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a new driver
    pub fn register_driver<D: Driver + 'static>(&mut self, mut driver: D) {
        let name = driver.name().to_string();
        
        // Auto-probe existing devices
        if let Some(all_pci) = try_get_all_pci_devices() {
            for pci_dev in all_pci {
                if driver.match_device(&pci_dev as &dyn Any) {
                    let _ = driver.probe(&pci_dev as &dyn Any);
                    self.attached_devices
                        .entry(name.clone())
                        .or_default()
                        .push(pci_dev);
                }
            }
        }
        
        self.drivers.insert(name.clone(), Box::new(driver));
    }
    
    /// Unregister a driver
    pub fn unregister_driver(&mut self, name: &str) -> Option<Box<dyn AnyDriver>> {
        self.drivers.remove(name)
    }
    
    /// Get info about attached devices
    pub fn get_attached_devices(&self, driver_name: &str) -> Option<&Vec<PciDevice>> {
        self.attached_devices.get(driver_name)
    }
    
    pub fn list_drivers(&self) -> Vec<&str> {
        self.drivers.keys().map(|s| s.as_str()).collect()
    }
}

// ============================================================================
// BUS TYPE SUPPORT
// ============================================================================

/// Bus type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusType {
    Pci,
    PciExpress,
    Usb,
    I2c,
    Spi,
    Uart,
    Platform,
    Mmc,
}

impl std::fmt::Display for BusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusType::Pci => write!(f, "PCI"),
            BusType::PciExpress => write!(f, "PCIe"),
            BusType::Usb => write!(f, "USB"),
            BusType::I2c => write!(f, "I2C"),
            BusType::Spi => write!(f, "SPI"),
            BusType::Uart => write!(f, "UART"),
            BusType::Platform => write!(f, "Platform"),
            BusType::Mmc => write!(f, "MMC"),
        }
    }
}

// ============================================================================
// HOTPLUG SUPPORT
// ============================================================================

/// Hotplug event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotplugEvent {
    DeviceAdded(BusType, u32),
    DeviceRemoved(BusType, u32),
    LinkUp(BusType, u32),
    LinkDown(BusType, u32),
}

/// Hotplug notifier callback
pub type HotplugCallback = Box<dyn Fn(HotplugEvent) + Send + Sync>;

#[derive(Debug)]
pub struct HotplugManager {
    callbacks: Vec<HotplugCallback>,
    registered_devices: HashMap<u32, BusType>,
}

impl Default for HotplugManager {
    fn default() -> Self {
        Self {
            callbacks: Vec::new(),
            registered_devices: HashMap::new(),
        }
    }
}

impl HotplugManager {
    pub fn notify(&self, event: HotplugEvent) {
        for callback in &self.callbacks {
            callback(event);
        }
    }
    
    pub fn register_callback<F>(&mut self, callback: F)
    where
        F: Fn(HotplugEvent) + Send + Sync + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }
    
    pub fn register_device(&mut self, id: u32, bus_type: BusType) {
        self.registered_devices.insert(id, bus_type);
    }
}

use std::collections::HashMap;
use crate::arch::cpu::*;

// Try to get all PCI devices (would query hardware in production)
fn try_get_all_pci_devices() -> Option<Vec<PciDevice>> {
    None
}

// ============================================================================
// DEVICE CLASSES FOR MATCHING
// ============================================================================

/// Standard PCI class codes
pub const PCI_CLASS_STORAGE: u32 = 0x010000;       // Storage controller
pub const PCI_CLASS_NETWORK: u32 = 0x020000;        // Network controller
pub const PCI_CLASS_DISPLAY: u32 = 0x030000;        // Display controller
pub const PCI_CLASS_MULTIMEDIA: u32 = 0x040000;     // Multimedia device
pub const PCI_CLASS_MEMORY: u32 = 0x050000;         // Memory controller
pub const PCI_CLASS_BRIDGE: u32 = 0x060000;         // Bridge device
pub const PCI_CLASS_COMMUNICATION: u32 = 0x070000;  // Communication controller
pub const PCI_CLASS_SANE: u32 = 0x080000;           // IEEE 1394
pub const PCI_CLASS_DIGITAL: u32 = 0x090000;        // DSP
pub const PCI_CLASS_ISA: u32 = 0x080100;            // ISA bridge

// ============================================================================
// TESTING UTILS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_driver_manager_registration() {
        let mut manager = DriverManager::new();
        
        // Create mock driver
        struct MockDriver;
        
        impl Driver for MockDriver {
            fn name(&self) -> &str { "mock" }
            fn match_device(&self, _: &dyn Any) -> bool { true }
            fn probe(&mut self, _: &dyn Any) -> Result<(), DriverError> { Ok(()) }
            fn remove(&mut self, _: &dyn Any) -> Result<(), DriverError> { Ok(()) }
        }
        
        manager.register_driver(MockDriver);
        
        assert!(manager.list_drivers().contains(&"mock"));
    }
    
    #[test]
    fn test_hotplug_notification() {
        let mut hotplug = HotplugManager::default();
        let mut notified = false;
        
        hotplug.register_callback(move |event| {
            notified = true;
            assert_eq!(event, HotplugEvent::DeviceAdded(BusType::Pci, 1));
        });
        
        hotplug.notify(HotplugEvent::DeviceAdded(BusType::Pci, 1));
        
        assert!(notified);
    }
    
    #[test]
    fn test_bus_type_display() {
        assert_eq!(format!("{}", BusType::Pci), "PCI");
        assert_eq!(format!("{}", BusType::Usb), "USB");
    }
}
