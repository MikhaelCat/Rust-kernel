use crate::system_profile::BootProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileRuntimeLimits {
    pub allow_net_tx: bool,
    pub allow_driver_optional: bool,
    pub allow_mount_syscalls: bool,
}

pub fn limits_for(profile: BootProfile) -> ProfileRuntimeLimits {
    match profile {
        BootProfile::Normal => ProfileRuntimeLimits {
            allow_net_tx: true,
            allow_driver_optional: true,
            allow_mount_syscalls: true,
        },
        BootProfile::Safe => ProfileRuntimeLimits {
            allow_net_tx: false,
            allow_driver_optional: true,
            allow_mount_syscalls: false,
        },
        BootProfile::Recovery => ProfileRuntimeLimits {
            allow_net_tx: false,
            allow_driver_optional: false,
            allow_mount_syscalls: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limits_match_profile() {
        let n = limits_for(BootProfile::Normal);
        assert!(n.allow_net_tx && n.allow_mount_syscalls);

        let s = limits_for(BootProfile::Safe);
        assert!(!s.allow_net_tx && !s.allow_mount_syscalls);

        let r = limits_for(BootProfile::Recovery);
        assert!(!r.allow_driver_optional);
    }
}
