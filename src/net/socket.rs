#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Socket {
    pub id: u64,
    pub open: bool,
    pub tx_packets: u64,
    pub rx_packets: u64,
    pub local_port: Option<u16>,
    pub peer_port: Option<u16>,
    pub listening: bool,
}

impl Socket {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            open: true,
            tx_packets: 0,
            rx_packets: 0,
            local_port: None,
            peer_port: None,
            listening: false,
        }
    }

    pub fn close(&mut self) {
        self.open = false;
    }
}
