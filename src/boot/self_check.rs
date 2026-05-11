use super::memory_map::MemoryMap;
use super::params::BootParams;
use super::policy::{BootPolicyResult, validate_boot_contract};

pub fn run_boot_self_check() -> bool {
    let params = BootParams::new("console=ttyS0 root=/dev/ram0", false);
    let mut map = MemoryMap::default();
    map.add_region(0, 2 * 1024 * 1024);
    validate_boot_contract(&params, &map, true) == BootPolicyResult::Pass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_self_check_passes() {
        assert!(run_boot_self_check());
    }
}
