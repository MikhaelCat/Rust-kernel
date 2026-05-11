use crate::arch::ArchSystem;
use crate::boot::BootParams;
use crate::boot::system::boot_smoke;
use crate::drivers::DriverSystem;
use crate::fs::FsSystem;
use crate::init::init_recovery_decision;
use crate::ipc::IpcSystem;
use crate::kernel::KernelSystem;
use crate::mm::MemoryManager;
use crate::net::NetSystem;
use crate::net::profile::config_for_profile;
use crate::power::PowerSystem;
use crate::security::health::check as security_check;
use crate::security::integration::security_may_enforce;
use crate::syscall::fault_matrix::validate_fault_matrix;
use crate::system_events::{EventBus, SystemEvent};
use crate::system_guard::{
    can_start_kernel, has_single_profile, no_duplicate_critical, validate_runtime_order,
};
use crate::system_health::SystemHealth;
use crate::system_profile::BootProfile;
use crate::system_profile_runtime::limits_for;
use crate::time::TimeSystem;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemReport {
    pub profile: BootProfile,
    pub boot_ok: bool,
    pub arch_ok: bool,
    pub pid: u32,
    pub net_bytes: usize,
    pub ipc_ok: bool,
    pub sec_ok: bool,
}

pub struct RustLinuxSystem {
    arch: ArchSystem,
    kernel: KernelSystem,
    drivers: DriverSystem,
    fs: FsSystem,
    net: NetSystem,
    ipc: IpcSystem,
    power: PowerSystem,
    time: TimeSystem,
    mm: MemoryManager,
    bus: EventBus,
}

impl RustLinuxSystem {
    pub fn new() -> Self {
        Self {
            arch: ArchSystem::default(),
            kernel: KernelSystem::new(),
            drivers: DriverSystem::default(),
            fs: FsSystem::new(),
            net: NetSystem::new(),
            ipc: IpcSystem::default(),
            power: PowerSystem::new(),
            time: TimeSystem::new(),
            mm: MemoryManager::new(4096 * 64, 4096).expect("mm init failed"),
            bus: EventBus::default(),
        }
    }

    pub fn boot_and_run(&mut self) -> SystemReport {
        self.boot_with_profile(BootProfile::Normal)
    }

    pub fn boot_with_failures(
        &mut self,
        boot_failures: u32,
        reason: Option<&'static str>,
    ) -> SystemReport {
        let decision = init_recovery_decision(boot_failures, reason);
        self.boot_with_profile(decision.profile)
    }

    pub fn boot_with_profile(&mut self, profile: BootProfile) -> SystemReport {
        self.bus.emit_once(SystemEvent::BootStarted);
        match profile {
            BootProfile::Normal => self.bus.emit_once(SystemEvent::ProfileNormal),
            BootProfile::Safe => self.bus.emit_once(SystemEvent::ProfileSafe),
            BootProfile::Recovery => self.bus.emit_once(SystemEvent::ProfileRecovery),
        }

        let params = BootParams::new("console=ttyS0 root=/dev/ram0", false);
        let mut img = vec![0u8; 64];
        img[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        img[24..32].copy_from_slice(&0x1000u64.to_le_bytes());
        let boot_ok = boot_smoke(&params.cmdline, &img);
        if boot_ok {
            self.bus.emit_once(SystemEvent::BootReady);
        }

        self.arch.bootstrap();
        let arch_ok = self.arch.paging_enabled();
        if arch_ok {
            self.bus.emit_once(SystemEvent::ArchReady);
        }

        self.power.bootstrap().expect("power bootstrap failed");
        self.bus.emit_once(SystemEvent::PowerReady);

        self.time.bootstrap().expect("time bootstrap failed");
        self.bus.emit_once(SystemEvent::TimeReady);

        let limits = limits_for(profile);

        self.drivers.load_base();
        if limits.allow_driver_optional {
            // Emulate optional drivers only when allowed by profile.
            self.drivers.load_base();
        }
        self.bus.emit_once(SystemEvent::DriversReady);

        self.fs.bootstrap();
        self.bus.emit_once(SystemEvent::FsReady);

        let _page = self.mm.map_new_page(0x4000).expect("map failed");
        self.bus.emit_once(SystemEvent::MmReady);

        assert!(can_start_kernel(self.bus.all()));
        self.kernel.bootstrap();
        let pid = self.kernel.tick();
        self.bus.emit_once(SystemEvent::KernelReady);

        self.net.bootstrap();
        let net_cfg = config_for_profile(profile);
        let net_bytes = if net_cfg.allow_tx && limits.allow_net_tx {
            self.net.smoke_send()
        } else {
            0
        };
        self.bus.emit_once(SystemEvent::NetReady);

        let ipc_ok = self.ipc.ping();
        if ipc_ok {
            self.bus.emit_once(SystemEvent::IpcReady);
        }

        let sec_ok = security_may_enforce(self.bus.all()) && security_check(true).enforcing;
        if sec_ok {
            self.bus.emit_once(SystemEvent::SecurityReady);
        }

        assert!(validate_fault_matrix());

        self.bus.emit_once(SystemEvent::SystemReady);

        assert!(self.bus.profile_selected());
        assert!(has_single_profile(self.bus.all()));
        assert!(no_duplicate_critical(self.bus.all()));
        assert!(validate_runtime_order(self.bus.all()));

        SystemReport {
            profile,
            boot_ok,
            arch_ok,
            pid,
            net_bytes,
            ipc_ok,
            sec_ok,
        }
    }

    pub fn health(&self, report: &SystemReport) -> SystemHealth {
        SystemHealth {
            boot_ok: report.boot_ok,
            arch_ok: report.arch_ok,
            kernel_ok: report.pid > 0,
            net_ok: report.net_bytes > 40 || report.profile.reduced_network(),
            ipc_ok: report.ipc_ok,
            security_ok: report.sec_ok,
        }
    }

    pub fn events(&self) -> &[SystemEvent] {
        self.bus.all()
    }
}

impl Default for RustLinuxSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_profile_flow() {
        let mut s = RustLinuxSystem::new();
        let r = s.boot_with_profile(BootProfile::Normal);
        assert_eq!(r.profile, BootProfile::Normal);
        assert!(r.net_bytes > 40);
        assert!(s.health(&r).all_ok());
    }

    #[test]
    fn safe_profile_reduces_network() {
        let mut s = RustLinuxSystem::new();
        let r = s.boot_with_profile(BootProfile::Safe);
        assert_eq!(r.net_bytes, 0);
        assert!(s.health(&r).all_ok());
    }

    #[test]
    fn recovery_profile_reduces_network() {
        let mut s = RustLinuxSystem::new();
        let r = s.boot_with_profile(BootProfile::Recovery);
        assert_eq!(r.net_bytes, 0);
        assert!(s.health(&r).all_ok());
    }

    #[test]
    fn failure_escalates_to_safe_profile() {
        let mut s = RustLinuxSystem::new();
        let r = s.boot_with_failures(3, Some("scheduler stall"));
        assert_eq!(r.profile, BootProfile::Safe);
        assert_eq!(r.net_bytes, 0);
        assert!(s.health(&r).all_ok());
    }

    #[test]
    fn severe_failure_escalates_to_recovery_profile() {
        let mut s = RustLinuxSystem::new();
        let r = s.boot_with_failures(7, Some("memory corruption in allocator"));
        assert_eq!(r.profile, BootProfile::Recovery);
        assert_eq!(r.net_bytes, 0);
        assert!(s.health(&r).all_ok());
    }
}
