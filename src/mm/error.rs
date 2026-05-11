#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MmError {
    OutOfMemory,
    InvalidPageIndex,
    DoubleFree,
    InvalidPageSize,
    InvalidRange,
    UnmappedAddress,
}
