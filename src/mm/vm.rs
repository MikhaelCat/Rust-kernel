use super::error::MmError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmArea {
    pub start: usize,
    pub end: usize,
}

impl VmArea {
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

#[derive(Debug, Default)]
pub struct VmaMap {
    areas: Vec<VmArea>,
}

impl VmaMap {
    pub fn insert(&mut self, start: usize, end: usize) -> Result<(), MmError> {
        if end <= start {
            return Err(MmError::InvalidRange);
        }
        self.areas.push(VmArea { start, end });
        Ok(())
    }

    pub fn count(&self) -> usize {
        self.areas.len()
    }
}
