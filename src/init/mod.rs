#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelConfig {
    pub name: &'static str,
    pub version: &'static str,
    pub page_size: usize,
    pub max_cpus: usize,
}

impl KernelConfig {
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.page_size == 0 || (self.page_size & (self.page_size - 1)) != 0 {
            return Err("page_size must be power of two");
        }
        if self.max_cpus == 0 {
            return Err("max_cpus must be > 0");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootStage {
    Reset,
    EarlyInit,
    MemoryInit,
    SchedulerInit,
    DeviceInit,
    Running,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootError {
    InvalidConfig(&'static str),
    StageFailure {
        stage: BootStage,
        reason: &'static str,
    },
}

#[derive(Debug, Clone)]
pub struct Kernel {
    pub config: KernelConfig,
    pub stage: BootStage,
    pub log: Vec<&'static str>,
}

impl Kernel {
    pub fn new(config: KernelConfig) -> Self {
        Self {
            config,
            stage: BootStage::Reset,
            log: Vec::new(),
        }
    }

    pub fn boot(&mut self) -> Result<(), BootError> {
        self.config.validate().map_err(BootError::InvalidConfig)?;

        self.transition(BootStage::EarlyInit, "early init");
        self.transition(BootStage::MemoryInit, "memory init");
        self.transition(BootStage::SchedulerInit, "scheduler init");
        self.transition(BootStage::DeviceInit, "device init");

        if self.config.max_cpus > 4096 {
            self.stage = BootStage::Failed;
            return Err(BootError::StageFailure {
                stage: BootStage::SchedulerInit,
                reason: "max_cpus exceeds supported limit",
            });
        }

        self.transition(BootStage::Running, "kernel running");
        Ok(())
    }

    fn transition(&mut self, next: BootStage, msg: &'static str) {
        self.stage = next;
        self.log.push(msg);
    }
}

pub fn default_config() -> KernelConfig {
    KernelConfig {
        name: "rust-linux",
        version: "0.1.0",
        page_size: 4096,
        max_cpus: 256,
    }
}

#[cfg(test)]
mod recovery_tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = default_config();
        assert_eq!(cfg.name, "rust-linux");
        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn rejects_non_power_of_two_page_size() {
        let mut cfg = default_config();
        cfg.page_size = 3000;
        assert!(cfg.validate().is_err());
    }

    #[test]
    fn boot_success_path_reaches_running() {
        let cfg = default_config();
        let mut kernel = Kernel::new(cfg);
        assert!(kernel.boot().is_ok());
        assert_eq!(kernel.stage, BootStage::Running);
        assert_eq!(
            kernel.log,
            vec![
                "early init",
                "memory init",
                "scheduler init",
                "device init",
                "kernel running"
            ]
        );
    }

    #[test]
    fn boot_fails_with_invalid_config() {
        let mut cfg = default_config();
        cfg.max_cpus = 0;
        let mut kernel = Kernel::new(cfg);
        let err = kernel.boot().expect_err("boot must fail");
        assert_eq!(err, BootError::InvalidConfig("max_cpus must be > 0"));
        assert_eq!(kernel.stage, BootStage::Reset);
    }

    #[test]
    fn boot_fails_when_cpu_limit_exceeded() {
        let mut cfg = default_config();
        cfg.max_cpus = 5000;
        let mut kernel = Kernel::new(cfg);
        let err = kernel.boot().expect_err("boot must fail");
        assert_eq!(
            err,
            BootError::StageFailure {
                stage: BootStage::SchedulerInit,
                reason: "max_cpus exceeds supported limit"
            }
        );
        assert_eq!(kernel.stage, BootStage::Failed);
    }
}
pub mod cmdline;
pub mod panic;

use crate::boot::recovery::{RecoveryMode, choose_recovery};
use crate::system_profile::BootProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitDecision {
    pub attempts: u32,
    pub recovery: RecoveryMode,
    pub profile: BootProfile,
    pub panic_severity: Option<&'static str>,
}

pub fn choose_profile_for_boot_failures(boot_failures: u32) -> BootProfile {
    if boot_failures >= 6 {
        BootProfile::Recovery
    } else if boot_failures >= 3 {
        BootProfile::Safe
    } else {
        BootProfile::Normal
    }
}

pub fn init_recovery_decision(boot_failures: u32, reason: Option<&'static str>) -> InitDecision {
    let recovery = choose_recovery(boot_failures);
    let profile = choose_profile_for_boot_failures(boot_failures);
    let panic_severity = reason.map(|r| panic::panic_report(r).severity);
    InitDecision {
        attempts: boot_failures,
        recovery,
        profile,
        panic_severity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_escalation_by_failures() {
        assert_eq!(choose_profile_for_boot_failures(0), BootProfile::Normal);
        assert_eq!(choose_profile_for_boot_failures(3), BootProfile::Safe);
        assert_eq!(choose_profile_for_boot_failures(6), BootProfile::Recovery);
    }

    #[test]
    fn recovery_decision_contains_panic_classification() {
        let d = init_recovery_decision(7, Some("memory corruption in early init"));
        assert_eq!(d.recovery, RecoveryMode::Safe);
        assert_eq!(d.profile, BootProfile::Recovery);
        assert_eq!(d.panic_severity, Some("critical"));
    }
}
