//! Physical Memory Manager Implementation

use super::error::MmError;

#[derive(Debug, Clone)]
pub struct PageFrame {
    pub index: usize,
    pub physical_address: usize,
    pub flags: u32,
}

impl PageFrame {
    pub fn new(index: usize, size: usize) -> Self {
        Self {
            index,
            physical_address: index * size,
            flags: 0,
        }
    }
}

#[derive(Debug)]
pub struct PhysicalMemoryManager {
    pub total_pages: u64,
    pub free_pages: Vec<usize>,
    pub used_pages: usize,
    pub page_size: usize,
}

impl Default for PhysicalMemoryManager {
    fn default() -> Self {
        Self::new(4096 * 1024, 4096).unwrap()
    }
}

impl PhysicalMemoryManager {
    pub fn new(total_bytes: usize, page_size: usize) -> Result<Self, MmError> {
        let total_pages = total_bytes / page_size;
        
        if total_bytes == 0 || page_size == 0 || total_bytes % page_size != 0 {
            return Err(MmError::InvalidParameter);
        }

        Ok(Self {
            total_pages: total_pages as u64,
            free_pages: (0..total_pages).collect(),
            used_pages: 0,
            page_size,
        })
    }

    pub fn allocate_page(&mut self) -> Result<PageFrame, MmError> {
        let page_index = self.free_pages.pop()
            .ok_or(MmError::NoMemory)?;
        
        self.used_pages += 1;
        Ok(PageFrame::new(page_index, self.page_size))
    }

    pub fn free_page(&mut self, page: PageFrame) -> Result<(), MmError> {
        if page.index >= self.total_pages as usize {
            return Err(MmError::InvalidAddress);
        }
        
        self.free_pages.push(page.index);
        self.used_pages -= 1;
        Ok(())
    }

    pub fn used_pages(&self) -> usize {
        self.used_pages
    }

    pub fn total_pages(&self) -> u64 {
        self.total_pages
    }

    pub fn free_count(&self) -> usize {
        self.free_pages.len()
    }

    pub fn available_memory(&self) -> usize {
        self.free_pages.len() * self.page_size
    }

    pub fn utilization(&self) -> f64 {
        if self.total_pages == 0 { return 0.0; }
        (self.used_pages as f64 / self.total_pages as f64) * 100.0
    }
}
