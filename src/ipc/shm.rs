#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShmSegment {
    pub id: u64,
    pub size: usize,
}
impl ShmSegment {
    pub fn new(id: u64, size: usize) -> Self {
        Self { id, size }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shm_new() {
        assert_eq!(ShmSegment::new(1, 4096).size, 4096);
    }
}
