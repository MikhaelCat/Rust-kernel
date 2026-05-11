#[derive(Debug, Default)]
pub struct MultiQueue {
    queues: u16,
}
impl MultiQueue {
    pub fn set_queues(&mut self, n: u16) {
        self.queues = n;
    }
    pub fn queues(&self) -> u16 {
        self.queues
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mq_set() {
        let mut m = MultiQueue::default();
        m.set_queues(4);
        assert_eq!(m.queues(), 4);
    }
}
