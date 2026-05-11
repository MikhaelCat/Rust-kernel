use super::{Ipv6Packet, NetDevice, NetworkStack, Route};

#[derive(Debug, Default)]
pub struct NetSystem {
    stack: NetworkStack,
}

impl NetSystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bootstrap(&mut self) {
        self.stack
            .add_device(NetDevice::new("eth0", 1500).expect("device create failed"));
        self.stack.add_route(Route::default_v6("eth0"));
    }

    pub fn smoke_send(&mut self) -> usize {
        let sid = self.stack.open_socket();
        let pkt = Ipv6Packet {
            src: [0; 16],
            dst: [1; 16],
            next_header: 17,
            payload: vec![1, 2, 3],
        };
        self.stack.send_ipv6(sid, &pkt).expect("send failed")
    }

    pub fn smoke_server_client(&mut self) -> bool {
        let server = self.stack.open_socket();
        self.stack.bind(server, 9090).expect("bind failed");
        self.stack.listen(server, 8).expect("listen failed");

        let client = self.stack.open_socket();
        self.stack.connect(client, 9090).expect("connect failed");
        let accepted = self.stack.accept(server).expect("accept failed");

        let sent = self.stack.send_payload(client, b"ipc-over-net");
        let recv = self.stack.recv_payload(accepted, 512);
        sent.is_ok() && recv.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_bootstrap_and_send() {
        let mut n = NetSystem::new();
        n.bootstrap();
        assert!(n.smoke_send() > 40);
        assert!(n.smoke_server_client());
    }
}
