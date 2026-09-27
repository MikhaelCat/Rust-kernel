//! VM (Virtual Memory) Subsystem - Virtual memory management

pub mod vma;
pub mod types;

// Re-export main types
pub use vma::{VmArea, VmaMap};
pub use types::Vm;
