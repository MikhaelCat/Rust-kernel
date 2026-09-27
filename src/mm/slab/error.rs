//! Slab Allocator Error Types

#[derive(Debug, Clone)]
pub enum SlabError {
    AllocationFailed,
    FreeInvalidPointer,
    CorruptedSlab,
    OutOfMemory,
}

impl std::fmt::Display for SlabError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllocationFailed => write!(f, "Slab allocation failed"),
            Self::FreeInvalidPointer => write!(f, "Free invalid pointer"),
            Self::CorruptedSlab => write!(f, "Slab corrupted"),
            Self::OutOfMemory => write!(f, "Out of memory"),
        }
    }
}

impl std::error::Error for SlabError {}
