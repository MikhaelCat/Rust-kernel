#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformError {
    FirmwareNotLoaded,
    AcpiNotLoaded,
    EfiRuntimeDisabled,
    DtbMissing,
}
