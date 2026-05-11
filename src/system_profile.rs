#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootProfile {
    Normal,
    Safe,
    Recovery,
}

impl BootProfile {
    pub fn reduced_network(self) -> bool {
        matches!(self, BootProfile::Safe | BootProfile::Recovery)
    }

    pub fn minimal_drivers(self) -> bool {
        matches!(self, BootProfile::Recovery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_flags() {
        assert!(!BootProfile::Normal.reduced_network());
        assert!(BootProfile::Safe.reduced_network());
        assert!(BootProfile::Recovery.minimal_drivers());
    }
}
