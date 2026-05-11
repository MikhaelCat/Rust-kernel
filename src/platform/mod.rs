pub mod acpi;
pub mod board;
pub mod dtb;
pub mod efi;
pub mod error;
pub mod firmware;
pub mod manager;
pub mod smbios;

pub use error::PlatformError;
pub use manager::PlatformManager;

pub mod fault;
pub mod system;
pub use system::{PlatformSnapshot, PlatformSystem};

pub mod compat;
pub mod self_check;

pub mod policy;
