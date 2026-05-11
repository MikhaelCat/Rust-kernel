#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformFault {
    FirmwareMissing,
    AcpiMissing,
    DtbInvalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformFaultAction {
    RetryInit,
    FallbackMinimal,
    Halt,
}

pub fn handle_platform_fault(f: PlatformFault) -> PlatformFaultAction {
    match f {
        PlatformFault::FirmwareMissing => PlatformFaultAction::RetryInit,
        PlatformFault::AcpiMissing => PlatformFaultAction::FallbackMinimal,
        PlatformFault::DtbInvalid => PlatformFaultAction::Halt,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fault_actions() {
        assert_eq!(
            handle_platform_fault(PlatformFault::FirmwareMissing),
            PlatformFaultAction::RetryInit
        );
        assert_eq!(
            handle_platform_fault(PlatformFault::AcpiMissing),
            PlatformFaultAction::FallbackMinimal
        );
        assert_eq!(
            handle_platform_fault(PlatformFault::DtbInvalid),
            PlatformFaultAction::Halt
        );
    }
}
