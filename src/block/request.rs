#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockRequest {
    pub id: u64,
    pub sector: u64,
    pub len: u32,
}
impl BlockRequest {
    pub fn new(id: u64, sector: u64, len: u32) -> Self {
        Self { id, sector, len }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn req_new() {
        assert_eq!(BlockRequest::new(1, 8, 4).len, 4);
    }
}
