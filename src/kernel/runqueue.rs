use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct RunQueue {
    q: VecDeque<u32>,
}

impl RunQueue {
    pub fn push(&mut self, pid: u32) {
        self.q.push_back(pid);
    }

    pub fn pop(&mut self) -> Option<u32> {
        self.q.pop_front()
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue_fifo() {
        let mut rq = RunQueue::default();
        rq.push(1);
        rq.push(2);
        assert_eq!(rq.pop(), Some(1));
        assert_eq!(rq.pop(), Some(2));
    }
}
