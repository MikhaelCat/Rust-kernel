use super::cqe::Cqe;
use super::op::IoOp;
use super::queue::{CompletionQueue, SubmissionQueue};
use super::sqe::Sqe;

#[derive(Debug, Default)]
pub struct IoUringEngine {
    sq_ids: SubmissionQueue,
    cq_ids: CompletionQueue,
    sqes: Vec<Sqe>,
    cqes: Vec<Cqe>,
    next_id: u64,
}

impl IoUringEngine {
    pub fn submit(&mut self, op: IoOp) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.sq_ids.push(id);
        self.sqes.push(Sqe { id, op, len: 0 });
        id
    }

    pub fn submit_with_len(&mut self, op: IoOp, len: u32) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.sq_ids.push(id);
        self.sqes.push(Sqe { id, op, len });
        id
    }

    pub fn poll_once(&mut self) -> Option<u64> {
        let id = self.sq_ids.pop()?;
        self.cq_ids.push(id);
        self.cqes.push(Cqe { id, res: 0 });
        self.cq_ids.pop()
    }

    pub fn complete_batch(&mut self) -> usize {
        let mut n = 0usize;
        while let Some(id) = self.sq_ids.pop() {
            self.cq_ids.push(id);
            self.cqes.push(Cqe { id, res: 0 });
            n += 1;
        }
        n
    }

    pub fn cqe_count(&self) -> usize {
        self.cqes.len()
    }

    pub fn submit_block_read(&mut self, len: u32) -> u64 {
        self.submit_with_len(IoOp::Read, len)
    }

    pub fn submit_block_write(&mut self, len: u32) -> u64 {
        self.submit_with_len(IoOp::Write, len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_and_complete() {
        let mut e = IoUringEngine::default();
        let id = e.submit(IoOp::Nop);
        assert_eq!(e.poll_once(), Some(id));
    }

    #[test]
    fn batch_completion() {
        let mut e = IoUringEngine::default();
        e.submit_with_len(IoOp::Read, 128);
        e.submit_with_len(IoOp::Write, 256);
        assert_eq!(e.complete_batch(), 2);
        assert_eq!(e.cqe_count(), 2);
    }
}
