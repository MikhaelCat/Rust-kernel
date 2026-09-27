//! Network Socket Implementation

use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};

#[derive(Debug)]
pub struct Socket {
    pub fd: i32,
    pub local_addr: SocketAddrV4,
    pub remote_addr: Option<SocketAddrV4>,
    pub state: SocketState,
    pub buffers: SocketBuffers,
}

#[derive(Debug, Clone)]
pub enum SocketState {
    Closed,
    Listening,
    Established,
    TimeWait,
}

#[derive(Debug, Default)]
pub struct SocketBuffers {
    pub send_buf: Vec<u8>,
    pub recv_buf: Vec<u8>,
}

impl Default for Socket {
    fn default() -> Self {
        Self::new()
    }
}

impl Socket {
    pub fn new(fd: i32, port: u16) -> Self {
        Self {
            fd,
            local_addr: SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port),
            remote_addr: None,
            state: SocketState::Closed,
            buffers: SocketBuffers::default(),
        }
    }

    pub fn bind(&mut self, addr: Ipv4Addr, port: u16) {
        self.local_addr = SocketAddrV4::new(addr, port);
    }

    pub fn listen(&mut self) {
        self.state = SocketState::Listening;
    }

    pub fn connect(&mut self, remote: SocketAddrV4) -> Result<(), &'static str> {
        if self.state != SocketState::Closed && self.state != SocketState::Listening {
            return Err("Invalid socket state");
        }
        self.remote_addr = Some(remote);
        self.state = SocketState::Established;
        Ok(())
    }

    pub fn accept(&mut self) -> Socket {
        let mut new_sock = Socket::new(self.fd + 1, self.remote_addr.unwrap().port());
        new_sock.state = SocketState::Established;
        new_sock
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        if self.state != SocketState::Established {
            return Err("Socket not established");
        }
        self.buffers.send_buf.extend_from_slice(data);
        Ok(data.len())
    }

    pub fn receive(&mut self, buf: &mut [u8]) -> Result<usize, &'static str> {
        if self.state != SocketState::Established {
            return Err("Socket not established");
        }
        let len = std::cmp::min(buf.len(), self.buffers.recv_buf.len());
        buf[..len].copy_from_slice(&self.buffers.recv_buf[..len]);
        Ok(len)
    }

    pub fn close(&mut self) {
        self.state = SocketState::Closed;
    }
}

#[derive(Debug)]
pub struct SocketTable {
    pub sockets: HashMap<i32, Socket>,
    pub next_fd: i32,
}

impl Default for SocketTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SocketTable {
    pub fn new() -> Self {
        Self {
            sockets: HashMap::new(),
            next_fd: 3,
        }
    }

    pub fn create_socket(&mut self) -> i32 {
        let fd = self.next_fd;
        self.next_fd += 1;
        
        let port = (fd * 10000) % 50000; // Simple port allocation
        let socket = Socket::new(fd, port as u16);
        
        self.sockets.insert(fd, socket);
        fd
    }

    pub fn get_socket(&self, fd: i32) -> Option<&Socket> {
        self.sockets.get(&fd)
    }

    pub fn get_socket_mut(&mut self, fd: i32) -> Option<&mut Socket> {
        self.sockets.get_mut(&fd)
    }
}
