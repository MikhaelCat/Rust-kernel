use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct SubmissionQueue {
    q: VecDeque<u64>,
}

impl SubmissionQueue {
    pub fn push(&mut self, id: u64) {
        self.q.push_back(id);
    }

    pub fn pop(&mut self) -> Option<u64> {
        self.q.pop_front()
    }

    pub fn len(&self) -> usize {
        self.q.len()
    }
}

#[derive(Debug, Default)]
pub struct CompletionQueue {
    q: VecDeque<u64>,
}

impl CompletionQueue {
    pub fn push(&mut self, id: u64) {
        self.q.push_back(id);
    }

    pub fn pop(&mut self) -> Option<u64> {
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
    fn sq_cq_fifo() {
        let mut sq = SubmissionQueue::default();
        sq.push(1);
        sq.push(2);
        assert_eq!(sq.pop(), Some(1));

        let mut cq = CompletionQueue::default();
        cq.push(10);
        assert_eq!(cq.pop(), Some(10));
    }
}
