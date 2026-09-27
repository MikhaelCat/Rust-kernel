//! Boot system module

#[derive(Debug, Clone, Default)]
pub struct BootSystem {
    pub running: bool,
}

impl BootSystem {
    pub fn new() -> Self {
        Self { running: false }
    }
}
