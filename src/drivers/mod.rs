pub mod registry;

pub use registry::{DriverError, DriverRegistry};
pub mod accel;
pub mod accel_fence;
pub mod accel_queue;
pub mod bus;
pub mod clock;
pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;
pub mod dma;
pub mod gpio;
pub mod i2c;
pub mod iommu;
pub mod mailbox;
pub mod pci;
pub mod regmap;
pub mod reset;
pub mod spi;
pub mod uart;

pub mod error;
pub mod manager;
pub use manager::DriverManager;

pub mod system;
pub use system::DriverSystem;

pub mod irqchip;
pub mod probe;
pub mod system2;

pub mod power_domain;

pub mod components;

pub mod fault;

pub mod recovery;

pub mod lifecycle;
