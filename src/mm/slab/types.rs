//! Slab Allocator Types

#[derive(Debug)]
pub struct SlabCache {
    pub name: String,
    pub object_size: usize,
    pub objects_per_slab: usize,
    pub free_objects: usize,
    pub total_objects: usize,
}

impl SlabCache {
    pub fn new(name: &str, object_size: usize) -> Self {
        Self {
            name: name.to_string(),
            object_size,
            objects_per_slab: 64,
            free_objects: 0,
            total_objects: 0,
        }
    }
    
    pub fn alloc(&mut self) -> Option<usize> {
        if self.free_objects > 0 {
            self.free_objects -= 1;
            Some(0) // In production, would return actual pointer
        } else {
            None
        }
    }
    
    pub fn free(&mut self, ptr: usize) {
        self.free_objects += 1;
    }
}
