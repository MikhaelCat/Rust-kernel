use super::error::MmError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFrame {
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct PhysicalMemoryManager {
    page_size: usize,
    total_pages: usize,
    allocated: Vec<bool>,
    used_pages: usize,
}

impl PhysicalMemoryManager {
    pub fn new(total_memory_bytes: usize, page_size: usize) -> Result<Self, MmError> {
        if page_size == 0 || (page_size & (page_size - 1)) != 0 {
            return Err(MmError::InvalidPageSize);
        }
        let total_pages = total_memory_bytes / page_size;
        Ok(Self {
            page_size,
            total_pages,
            allocated: vec![false; total_pages],
            used_pages: 0,
        })
    }

    pub fn allocate_page(&mut self) -> Result<PageFrame, MmError> {
        for i in 0..self.total_pages {
            if !self.allocated[i] {
                self.allocated[i] = true;
                self.used_pages += 1;
                return Ok(PageFrame { index: i });
            }
        }
        Err(MmError::OutOfMemory)
    }

    pub fn free_page(&mut self, page: PageFrame) -> Result<(), MmError> {
        if page.index >= self.total_pages {
            return Err(MmError::InvalidPageIndex);
        }
        if !self.allocated[page.index] {
            return Err(MmError::DoubleFree);
        }
        self.allocated[page.index] = false;
        self.used_pages -= 1;
        Ok(())
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }
    pub fn total_pages(&self) -> usize {
        self.total_pages
    }
    pub fn used_pages(&self) -> usize {
        self.used_pages
    }
    pub fn free_pages(&self) -> usize {
        self.total_pages - self.used_pages
    }
}
