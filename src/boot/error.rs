#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BootError {
    InvalidCmdline,
    InvalidMemoryMap,
    InvalidElfImage,
    MissingKernelImage,
}
