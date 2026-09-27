//! Security Subsystem - LSM Framework & SELinux Policy Engine
//! 
//! Реализация Linux Security Modules (LSM) framework и SELinux policy engine
//! для принудительного контроля доступа в ядре Linux на Rust

pub mod lsm;
pub mod selinux;

// Re-export commonly used types
pub use crate::security::lsm::{
    AccessError, AccessVector, Capabilities, ObjectCategory, Permission, Permissions,
    SecurityHook, SecurityLabel, SecurityPolicy, SecuritySubsystem, SyscallContext,
    TaskSecurityContext, RlimitType, AvcStats,
};

pub use crate::security::selinux::SelinuxPolicy;

/// Security subsystem statistics
#[derive(Debug, Clone, Default)]
pub struct SecurityStats {
    pub checks_total: u64,
    pub checks_passed: u64,
    pub checks_denied: u64,
}

/// Integration hooks for VFS layer
pub mod vfs_hooks {
    use super::*;
    
    /// Check file open with security policy
    pub fn check_file_open(security: &SecuritySubsystem, path: &str, flags: i32) -> Result<(), AccessError> {
        if !security.enabled {
            return Ok(());
        }
        
        security.check(&SecurityHook::FileOpen(path.to_string(), flags))
    }
    
    /// Check file creation with security policy
    pub fn check_file_create(security: &SecuritySubsystem, path: &str, mode: u32) -> Result<(), AccessError> {
        if !security.enabled {
            return Ok(());
        }
        
        security.check(&SecurityHook::FileCreate(path.to_string(), mode as u32))
    }
    
    /// Check directory creation with security policy
    pub fn check_mkdir(security: &SecuritySubsystem, path: &str) -> Result<(), AccessError> {
        if !security.enabled {
            return Ok(());
        }
        
        security.check(&SecurityHook::FileMkdir(path.to_string(), 0o755))
    }
}

/// Integration hooks for Network stack
pub mod net_hooks {
    use super::*;
    
    /// Check socket bind with security policy
    pub fn check_socket_bind(
        security: &SecuritySubsystem, 
        addr: std::net::SocketAddrV4
    ) -> Result<(), AccessError> {
        if !security.enabled {
            return Ok(());
        }
        
        security.check(&SecurityHook::SocketBind(addr))
    }
    
    /// Check socket connect with security policy
    pub fn check_socket_connect(
        security: &SecuritySubsystem,
        addr: std::net::SocketAddrV4
    ) -> Result<(), AccessError> {
        if !security.enabled {
            return Ok(());
        }
        
        security.check(&SecurityHook::SocketConnect(addr))
    }
    
    /// Check privileged port binding
    pub fn check_privileged_port(port: u16, capabilities: &Capabilities) -> Result<(), AccessError> {
        if port < 1024 && !capabilities.has(Capabilities::CAP_NET_BIND_SERVICE) {
            Err(AccessError::CapabilityDenied)
        } else {
            Ok(())
        }
    }
}

/// Test helpers
#[cfg(test)]
mod test_helpers {
    use super::*;
    
    /// Create test security subsystem with basic policy
    pub fn create_test_security() -> SecuritySubsystem {
        let mut security = SecuritySubsystem::new();
        let policy = SelinuxPolicy::new();
        security.register_policy(Box::new(policy));
        security.enable();
        security
    }
}
