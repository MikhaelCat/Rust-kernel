use std::collections::BTreeMap;

use super::error::MmError;

#[derive(Debug, Default)]
pub struct PageTable {
    mappings: BTreeMap<usize, usize>,
}

impl PageTable {
    pub fn map(&mut self, va: usize, pa: usize) {
        self.mappings.insert(va, pa);
    }

    pub fn unmap(&mut self, va: usize) -> Result<(), MmError> {
        if self.mappings.remove(&va).is_none() {
            return Err(MmError::UnmappedAddress);
        }
        Ok(())
    }

    pub fn translate(&self, va: usize) -> Option<usize> {
        self.mappings.get(&va).copied()
    }
}
