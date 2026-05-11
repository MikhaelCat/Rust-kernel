use super::acpi::Acpi;
use super::efi::Efi;
use super::firmware::Firmware;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPolicyResult {
    Pass,
    Fail,
}

pub fn validate_boot_contract(fw: &Firmware, acpi: &Acpi, efi: &Efi) -> PlatformPolicyResult {
    let fw_ok = fw.loaded() && fw.meta().is_some();
    let acpi_ok = acpi.has_table("FADT") && acpi.has_table("MADT");
    let efi_ok = efi.runtime && efi.get_var("BootCurrent").is_some();

    if fw_ok && acpi_ok && efi_ok {
        PlatformPolicyResult::Pass
    } else {
        PlatformPolicyResult::Fail
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::{acpi::Acpi, efi::Efi, firmware::Firmware};

    #[test]
    fn policy_passes_when_contract_met() {
        let mut fw = Firmware::default();
        fw.load();
        let mut acpi = Acpi::default();
        acpi.load();
        let mut efi = Efi::default();
        efi.enable_runtime();
        efi.set_var("BootCurrent", "0001");
        assert_eq!(
            validate_boot_contract(&fw, &acpi, &efi),
            PlatformPolicyResult::Pass
        );
    }
}
