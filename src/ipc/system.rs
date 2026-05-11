use super::IpcManager;

#[derive(Debug, Default)]
pub struct IpcSystem {
    ipc: IpcManager,
}

impl IpcSystem {
    pub fn ping(&mut self) -> bool {
        self.ipc.send(b"ping");
        self.ipc.recv().map(|m| m == b"ping").unwrap_or(false)
    }

    pub fn exchange_on_channel(&mut self, chan: u32, payload: &[u8]) -> bool {
        self.ipc.send_to(chan, payload);
        self.ipc
            .recv_from(chan)
            .map(|m| m == payload)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_roundtrip() {
        let mut i = IpcSystem::default();
        assert!(i.ping());
    }

    #[test]
    fn channel_roundtrip() {
        let mut i = IpcSystem::default();
        assert!(i.exchange_on_channel(1, b"hello"));
    }
}
