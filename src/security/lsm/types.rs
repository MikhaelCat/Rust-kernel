//! Linux Security Module (LSM) types and structures

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// LSM hook types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmHook {
    FileOpen,
    FileCreate,
    FileRename,
    MmapProtect,
    SocketBind,
    ProcessExec,
    ProcessFork,
    CredAlloc,
    CredFree,
    NetListen,
}

/// Security policy state
#[derive(Debug)]
pub struct SecurityPolicyState {
    pub enabled: AtomicBool,
    pub enforcing: bool,
    pub allow_discretionary: bool,
}

impl Default for SecurityPolicyState {
    fn default() -> Self {
        Self {
            enabled: AtomicBool::new(true),
            enforcing: true,
            allow_discretionary: false,
        }
    }
}

/// LSM hook registry
#[derive(Debug)]
pub struct HookRegistry {
    pub registered_hooks: AtomicUsize,
    pub active_policies: Vec<String>,
    pub callback_invocations: AtomicUsize,
}

impl HookRegistry {
    pub fn new() -> Self {
        Self {
            registered_hooks: AtomicUsize::new(0),
            active_policies: vec![],
            callback_invocations: AtomicUsize::new(0),
        }
    }
}

/// Capability flags
#[derive(Debug)]
pub struct CapabilityFlags {
    pub cap_ipc_lock: bool,
    pub cap_sys_admin: bool,
    pub cap_net_bind_service: bool,
    pub cap_mknod: bool,
    pub cap_chown: bool,
    pub cap_fowner: bool,
    pub cap_fsetid: bool,
}

impl Default for CapabilityFlags {
    fn default() -> Self {
        Self {
            cap_ipc_lock: false,
            cap_sys_admin: false,
            cap_net_bind_service: false,
            cap_mknod: false,
            cap_chown: false,
            cap_fowner: false,
            cap_fsetid: false,
        }
    }
}
