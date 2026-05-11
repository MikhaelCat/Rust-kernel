use super::channel::MessageQueue;
use super::error::IpcCoreError;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct IpcManager {
    queue: MessageQueue,
    channels: BTreeMap<u32, MessageQueue>,
}

impl IpcManager {
    pub fn send(&mut self, msg: &[u8]) {
        self.queue.send(msg);
    }

    pub fn recv(&mut self) -> Result<Vec<u8>, IpcCoreError> {
        self.queue.recv().map_err(|_| IpcCoreError::QueueEmpty)
    }

    pub fn send_to(&mut self, chan: u32, msg: &[u8]) {
        self.channels.entry(chan).or_default().send(msg);
    }

    pub fn recv_from(&mut self, chan: u32) -> Result<Vec<u8>, IpcCoreError> {
        self.channels
            .get_mut(&chan)
            .ok_or(IpcCoreError::NoSuchChannel)?
            .recv()
            .map_err(|_| IpcCoreError::QueueEmpty)
    }

    pub fn dispatch_batch(&mut self, chan: u32, batch: &[Vec<u8>]) -> usize {
        for msg in batch {
            self.send_to(chan, msg);
        }
        batch.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_recv_flow() {
        let mut ipc = IpcManager::default();
        ipc.send(b"ping");
        assert_eq!(ipc.recv().expect("recv failed"), b"ping");
    }

    #[test]
    fn channel_flow() {
        let mut ipc = IpcManager::default();
        ipc.send_to(7, b"one");
        ipc.send_to(7, b"two");
        assert_eq!(ipc.recv_from(7).expect("recv1"), b"one");
        assert_eq!(ipc.recv_from(7).expect("recv2"), b"two");
    }
}
