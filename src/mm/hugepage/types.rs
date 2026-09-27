//! Types for hugepage memory management

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Huge page size configurations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HugePageSize {
    TwoMb = 2 << 20,
    OneGb = 1 << 30,
}

impl HugePageSize {
    pub const fn size(&self) -> usize {
        match self {
            HugePageSize::TwoMb => 2 * 1024 * 1024,
            HugePageSize::OneGb => 1024 * 1024 * 1024,
        }
    }
}

/// Transparent hugepage state
#[derive(Debug)]
pub struct TransparentHugePage {
    pub enabled: AtomicBool,
    pub defered_compound: AtomicBool,
    pub backend_supported: bool,
}

impl Default for TransparentHugePage {
    fn default() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            defered_compound: AtomicBool::new(false),
            backend_supported: true,
        }
    }
}

/// Huge page allocation statistics
#[derive(Debug)]
pub struct HugePageStats {
    pub total_hugepages: AtomicUsize,
    pub free_hugepages: AtomicUsize,
    pub reserved_hugepages: AtomicUsize,
    pub surpplus_hugepages: AtomicUsize,
}

impl HugePageStats {
    pub fn new() -> Self {
        Self {
            total_hugepages: AtomicUsize::new(0),
            free_hugepages: AtomicUsize::new(0),
            reserved_hugepages: AtomicUsize::new(0),
            surpplus_hugepages: AtomicUsize::new(0),
        }
    }
}
