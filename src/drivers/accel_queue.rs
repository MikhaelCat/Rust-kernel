#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccelQueue {
    pub id: u32,
    pub depth: u32,
}
impl AccelQueue {
    pub fn new(id: u32, depth: u32) -> Option<Self> {
        if depth == 0 {
            None
        } else {
            Some(Self { id, depth })
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_valid() {
        assert!(AccelQueue::new(0, 128).is_some());
        assert!(AccelQueue::new(0, 0).is_none());
    }
}
