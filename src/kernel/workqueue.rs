#[derive(Debug, Default)]
pub struct WorkQueue {
    queued: u64,
}
impl WorkQueue {
    pub fn queue_work(&mut self) {
        self.queued += 1;
    }
    pub fn queued(&self) -> u64 {
        self.queued
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_one() {
        let mut w = WorkQueue::default();
        w.queue_work();
        assert_eq!(w.queued(), 1);
    }
}
