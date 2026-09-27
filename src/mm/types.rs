//! MM (Memory Management) statistics and types

#[derive(Debug, Clone, Default)]
pub struct VmaManager {
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct Vma {
    pub start: usize,
    pub end: usize,
    pub flags: u32,
}
