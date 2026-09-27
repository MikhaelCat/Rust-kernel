//! SELinux security module types

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// SELinux security context
#[derive(Debug)]
pub struct SecurityContext {
    pub user: String,
    pub role: String,
    pub object_type: String,
    pub level: String,
}

impl Default for SecurityContext {
    fn default() -> Self {
        Self {
            user: "unspecified".to_string(),
            role: "object_r".to_string(),
            object_type: "default".to_string(),
            level: "s0".to_string(),
        }
    }
}

/// SELinux policy state
#[derive(Debug)]
pub struct SelinuxPolicyState {
    pub enabled: AtomicBool,
    pub enforcing: bool,
    pub check_access_calls: AtomicUsize,
    pub avc_cache_size: usize,
}

impl Default for SelinuxPolicyState {
    fn default() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            enforcing: true,
            check_access_calls: AtomicUsize::new(0),
            avc_cache_size: 512,
        }
    }
}

/// SELinux policy configuration
#[derive(Debug)]
pub struct SelinuxConfig {
    pub strict_checking: bool,
    pub audit_disabled: bool,
    pub permissive_mode: bool,
}

impl Default for SelinuxConfig {
    fn default() -> Self {
        Self {
            strict_checking: false,
            audit_disabled: false,
            permissive_mode: false,
        }
    }
}
