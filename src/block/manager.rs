//! Block device manager

#[derive(Debug, Clone)]
pub struct BlockManager {
    devices: Vec<String>,
}

impl Default for BlockManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockManager {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }
}
