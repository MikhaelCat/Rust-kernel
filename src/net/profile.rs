use crate::system_profile::BootProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetProfileConfig {
    pub allow_tx: bool,
    pub allow_control_plane: bool,
}

pub fn config_for_profile(profile: BootProfile) -> NetProfileConfig {
    match profile {
        BootProfile::Normal => NetProfileConfig {
            allow_tx: true,
            allow_control_plane: true,
        },
        BootProfile::Safe => NetProfileConfig {
            allow_tx: false,
            allow_control_plane: true,
        },
        BootProfile::Recovery => NetProfileConfig {
            allow_tx: false,
            allow_control_plane: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_network_behavior() {
        assert!(config_for_profile(BootProfile::Normal).allow_tx);
        assert!(!config_for_profile(BootProfile::Safe).allow_tx);
        assert!(!config_for_profile(BootProfile::Recovery).allow_control_plane);
    }
}
