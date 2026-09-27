//! VMA (Virtual Memory Area) management

#[derive(Debug, Clone)]
pub struct VmArea {
    pub start: usize,
    pub end: usize,
    pub flags: u32,
}

#[derive(Debug, Clone)]
pub struct VmaMap {
    pub areas: Vec<VmArea>,
}

impl VmaMap {
    pub fn new() -> Self {
        Self { areas: Vec::new() }
    }
    
    pub fn insert(&mut self, area: VmArea) {
        self.areas.push(area);
    }
}

impl Default for VmaMap {
    fn default() -> Self {
        Self::new()
    }
}
