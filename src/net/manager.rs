use std::collections::BTreeMap;
use std::collections::VecDeque;

use super::device::NetDevice;
use super::error::NetError;
use super::packet::{Ipv4Packet, Ipv6Packet};
use super::route::Route;
use super::socket_table::SocketTable;

#[derive(Debug, Default)]
pub struct NetworkStack {
    devices: BTreeMap<String, NetDevice>,
    routes: Vec<Route>,
    sockets: SocketTable,
    listeners: BTreeMap<u16, VecDeque<u16>>,
    tx_bytes: usize,
    rx_bytes: usize,
}

impl NetworkStack {
    pub fn add_device(&mut self, dev: NetDevice) {
        self.devices.insert(dev.name.clone(), dev);
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn open_socket(&mut self) -> u64 {
        self.sockets.open()
    }

    pub fn send_ipv6(&mut self, socket_id: u64, packet: &Ipv6Packet) -> Result<usize, NetError> {
        let route = self.routes.first().ok_or(NetError::RouteNotFound)?;
        let dev = self
            .devices
            .get(&route.dev)
            .ok_or(NetError::DeviceNotFound)?;
        if !dev.up {
            return Err(NetError::DeviceNotFound);
        }

        let socket = self
            .sockets
            .get_mut(socket_id)
            .ok_or(NetError::SocketNotFound)?;
        if !socket.open {
            return Err(NetError::SocketClosed);
        }

        let raw = packet.encode()?;
        if raw.len() > dev.mtu {
            return Err(NetError::PayloadTooLarge);
        }

        socket.tx_packets += 1;
        self.tx_bytes += raw.len();
        Ok(raw.len())
    }

    pub fn send_ipv4(&mut self, socket_id: u64, packet: &Ipv4Packet) -> Result<usize, NetError> {
        let route = self.routes.first().ok_or(NetError::RouteNotFound)?;
        let dev = self
            .devices
            .get(&route.dev)
            .ok_or(NetError::DeviceNotFound)?;
        if !dev.up {
            return Err(NetError::DeviceNotFound);
        }

        let socket = self
            .sockets
            .get_mut(socket_id)
            .ok_or(NetError::SocketNotFound)?;
        if !socket.open {
            return Err(NetError::SocketClosed);
        }

        let raw = packet.encode()?;
        if raw.len() > dev.mtu {
            return Err(NetError::PayloadTooLarge);
        }

        socket.tx_packets += 1;
        self.tx_bytes += raw.len();
        Ok(raw.len())
    }

    pub fn bind(&mut self, socket_id: u64, port: u16) -> Result<(), NetError> {
        let socket = self
            .sockets
            .get_mut(socket_id)
            .ok_or(NetError::SocketNotFound)?;
        if !socket.open {
            return Err(NetError::SocketClosed);
        }
        socket.local_port = Some(port);
        Ok(())
    }

    pub fn listen(&mut self, socket_id: u64, _backlog: usize) -> Result<(), NetError> {
        let socket = self
            .sockets
            .get_mut(socket_id)
            .ok_or(NetError::SocketNotFound)?;
        let port = socket.local_port.ok_or(NetError::NotBound)?;
        socket.listening = true;
        self.listeners.entry(port).or_default();
        Ok(())
    }

    pub fn connect(&mut self, socket_id: u64, remote_port: u16) -> Result<(), NetError> {
        let socket = self
            .sockets
            .get_mut(socket_id)
            .ok_or(NetError::SocketNotFound)?;
        if !socket.open {
            return Err(NetError::SocketClosed);
        }
        socket.peer_port = Some(remote_port);
        let queue = self
            .listeners
            .get_mut(&remote_port)
            .ok_or(NetError::NotListening)?;
        queue.push_back(socket_id as u16);
        Ok(())
    }

    pub fn accept(&mut self, socket_id: u64) -> Result<u64, NetError> {
        let socket = self
            .sockets
            .get(socket_id)
            .ok_or(NetError::SocketNotFound)?;
        let port = socket.local_port.ok_or(NetError::NotBound)?;
        if !socket.listening {
            return Err(NetError::NotListening);
        }
        let _peer = self
            .listeners
            .get_mut(&port)
            .ok_or(NetError::NotListening)?
            .pop_front()
            .ok_or(NetError::NoPendingConnection)?;
        let new_id = self.sockets.open();
        let accepted = self
            .sockets
            .get_mut(new_id)
            .ok_or(NetError::SocketNotFound)?;
        accepted.local_port = Some(port);
        accepted.peer_port = Some(port);
        Ok(new_id)
    }

    pub fn send_payload(&mut self, socket_id: u64, data: &[u8]) -> Result<usize, NetError> {
        let packet = Ipv6Packet {
            src: [0; 16],
            dst: [1; 16],
            next_header: 17,
            payload: data.to_vec(),
        };
        self.send_ipv6(socket_id, &packet)
    }

    pub fn recv_payload(&mut self, socket_id: u64, max_len: usize) -> Result<usize, NetError> {
        let socket = self
            .sockets
            .get_mut(socket_id)
            .ok_or(NetError::SocketNotFound)?;
        if !socket.open {
            return Err(NetError::SocketClosed);
        }
        let got = max_len.min(1500);
        socket.rx_packets += 1;
        self.rx_bytes += got;
        Ok(got)
    }

    pub fn counters(&self) -> (usize, usize) {
        (self.tx_bytes, self.rx_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pkt(payload_len: usize) -> Ipv6Packet {
        Ipv6Packet {
            src: [0; 16],
            dst: [1; 16],
            next_header: 17,
            payload: vec![0xAB; payload_len],
        }
    }

    #[test]
    fn network_send_happy_path() {
        let mut net = NetworkStack::default();
        net.add_device(NetDevice::new("eth0", 1500).expect("dev create"));
        net.add_route(Route::default_v6("eth0"));
        let sid = net.open_socket();

        let sent = net.send_ipv6(sid, &pkt(100)).expect("send failed");
        assert!(sent > 40);
    }

    #[test]
    fn fails_without_route() {
        let mut net = NetworkStack::default();
        net.add_device(NetDevice::new("eth0", 1500).expect("dev create"));
        let sid = net.open_socket();
        assert_eq!(net.send_ipv6(sid, &pkt(10)), Err(NetError::RouteNotFound));
    }

    #[test]
    fn fails_for_unknown_socket() {
        let mut net = NetworkStack::default();
        net.add_device(NetDevice::new("eth0", 1500).expect("dev create"));
        net.add_route(Route::default_v6("eth0"));
        assert_eq!(net.send_ipv6(999, &pkt(10)), Err(NetError::SocketNotFound));
    }

    #[test]
    fn bind_listen_connect_accept_flow() {
        let mut net = NetworkStack::default();
        net.add_device(NetDevice::new("eth0", 1500).expect("dev create"));
        net.add_route(Route::default_v6("eth0"));

        let server = net.open_socket();
        net.bind(server, 8080).expect("bind");
        net.listen(server, 16).expect("listen");

        let client = net.open_socket();
        net.connect(client, 8080).expect("connect");
        let accepted = net.accept(server).expect("accept");
        assert!(accepted > 0);

        let sent = net.send_payload(client, b"hello").expect("send");
        let recv = net.recv_payload(accepted, 256).expect("recv");
        assert!(sent > 40);
        assert!(recv > 0);
        let (tx, rx) = net.counters();
        assert!(tx >= sent);
        assert!(rx >= recv);
    }

    #[test]
    fn ipv4_send_happy_path() {
        let mut net = NetworkStack::default();
        net.add_device(NetDevice::new("eth0", 1500).expect("dev"));
        net.add_route(Route::default_v6("eth0"));
        let sid = net.open_socket();
        let p = Ipv4Packet {
            src: [192, 168, 0, 2],
            dst: [192, 168, 0, 3],
            protocol: 17,
            payload: vec![0x11; 64],
        };
        let sent = net.send_ipv4(sid, &p).expect("send4");
        assert!(sent >= 84);
    }
}
