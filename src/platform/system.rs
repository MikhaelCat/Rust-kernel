use super::acpi::Acpi;
use super::board::Board;
use super::compat::{CompatResult, check_board_firmware};
use super::dtb::DtbBlob;
use super::efi::Efi;
use super::firmware::Firmware;
use super::manager::PlatformManager;
use super::policy::{PlatformPolicyResult, validate_boot_contract};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformSnapshot {
    pub board: &'static str,
    pub firmware_loaded: bool,
    pub firmware_vendor: Option<String>,
    pub acpi_loaded: bool,
    pub acpi_tables: usize,
    pub efi_runtime: bool,
    pub dtb_size: usize,
    pub compatible: bool,
    pub policy_pass: bool,
    pub ready: bool,
}

#[derive(Debug)]
pub struct PlatformSystem {
    board: Board,
    fw: Firmware,
    acpi: Acpi,
    efi: Efi,
    dtb: DtbBlob,
    mgr: PlatformManager,
}

impl PlatformSystem {
    pub fn new(board: Board, dtb_bytes: &[u8]) -> Self {
        Self {
            board: board.clone(),
            fw: Firmware::default(),
            acpi: Acpi::default(),
            efi: Efi::default(),
            dtb: DtbBlob::parse(dtb_bytes),
            mgr: PlatformManager::new(board),
        }
    }

    pub fn bootstrap(&mut self) {
        self.fw.load();
        self.acpi.load();
        self.efi.enable_runtime();
        self.efi.set_var("BootCurrent", "0001");
        self.mgr.load_firmware();
    }

    pub fn snapshot(&self) -> PlatformSnapshot {
        let compat = check_board_firmware(&self.board, &self.fw) == CompatResult::Compatible;
        let policy_pass =
            validate_boot_contract(&self.fw, &self.acpi, &self.efi) == PlatformPolicyResult::Pass;

        PlatformSnapshot {
            board: self.board.name,
            firmware_loaded: self.fw.loaded(),
            firmware_vendor: self.fw.meta().map(|m| m.vendor.clone()),
            acpi_loaded: self.acpi.tables_loaded,
            acpi_tables: self.acpi.table_count(),
            efi_runtime: self.efi.runtime,
            dtb_size: self.dtb.size,
            compatible: compat,
            policy_pass,
            ready: self.mgr.boot_ready().is_ok(),
        }
    }

    pub fn boot_contract_ready(&self) -> Result<(), super::error::PlatformError> {
        if !self.fw.loaded() {
            return Err(super::error::PlatformError::FirmwareNotLoaded);
        }
        if !self.acpi.tables_loaded {
            return Err(super::error::PlatformError::AcpiNotLoaded);
        }
        if !self.efi.runtime {
            return Err(super::error::PlatformError::EfiRuntimeDisabled);
        }
        if self.dtb.size == 0 {
            return Err(super::error::PlatformError::DtbMissing);
        }
        self.mgr.boot_ready().map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_bootstrap_flow() {
        let mut p = PlatformSystem::new(Board::generic(), &[1, 2, 3, 4]);
        p.bootstrap();
        let s = p.snapshot();
        assert_eq!(s.board, "generic");
        assert!(s.firmware_loaded);
        assert_eq!(s.firmware_vendor.as_deref(), Some("rust-linux"));
        assert!(s.acpi_loaded);
        assert_eq!(s.acpi_tables, 3);
        assert!(s.efi_runtime);
        assert_eq!(s.dtb_size, 4);
        assert!(s.compatible);
        assert!(s.policy_pass);
        assert!(s.ready);
        assert!(p.boot_contract_ready().is_ok());
    }

    #[test]
    fn contract_rejects_missing_dtb() {
        let mut p = PlatformSystem::new(Board::generic(), &[]);
        p.bootstrap();
        assert_eq!(
            p.boot_contract_ready(),
            Err(super::super::error::PlatformError::DtbMissing)
        );
    }
}
