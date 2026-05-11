#[derive(Debug, Default)]
pub struct BlockQueue {
    depth: usize,
}
impl BlockQueue {
    pub fn submit(&mut self) {
        self.depth += 1;
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn submit_req() {
        let mut q = BlockQueue::default();
        q.submit();
        assert_eq!(q.depth(), 1);
    }
}
