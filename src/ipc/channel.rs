use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcError {
    Empty,
}

#[derive(Debug, Default)]
pub struct MessageQueue {
    queue: VecDeque<Vec<u8>>,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn send(&mut self, msg: &[u8]) {
        self.queue.push_back(msg.to_vec());
    }

    pub fn recv(&mut self) -> Result<Vec<u8>, IpcError> {
        self.queue.pop_front().ok_or(IpcError::Empty)
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_delivery() {
        let mut q = MessageQueue::new();
        q.send(b"one");
        q.send(b"two");
        assert_eq!(q.recv().expect("first"), b"one");
        assert_eq!(q.recv().expect("second"), b"two");
    }
}
