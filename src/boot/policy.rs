use super::memory_map::MemoryMap;
use super::params::BootParams;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPolicyResult {
    Pass,
    Fail,
}

pub fn validate_boot_contract(
    params: &BootParams,
    map: &MemoryMap,
    image_present: bool,
) -> BootPolicyResult {
    let cmd_ok = params.validate() && params.get("console").is_some();
    let mem_ok = map.is_valid() && map.total_size() >= 1024 * 1024;
    let image_ok = image_present;

    if cmd_ok && mem_ok && image_ok {
        BootPolicyResult::Pass
    } else {
        BootPolicyResult::Fail
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boot::params::BootParams;

    #[test]
    fn boot_policy_passes_with_valid_contract() {
        let p = BootParams::new("console=ttyS0 root=/dev/ram0", false);
        let mut m = MemoryMap::default();
        m.add_region(0, 2 * 1024 * 1024);
        assert_eq!(validate_boot_contract(&p, &m, true), BootPolicyResult::Pass);
    }
}
