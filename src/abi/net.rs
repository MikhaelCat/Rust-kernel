use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct SocketTable {
    next_fd: i64,
    connected: BTreeMap<i64, bool>,
}

impl SocketTable {
    pub fn new() -> Self {
        Self {
            next_fd: 100,
            connected: BTreeMap::new(),
        }
    }

    pub fn socket(&mut self) -> i64 {
        let fd = self.next_fd;
        self.next_fd += 1;
        self.connected.insert(fd, false);
        fd
    }

    pub fn connect(&mut self, fd: i64) -> i64 {
        match self.connected.get_mut(&fd) {
            Some(v) => {
                *v = true;
                0
            }
            None => -9,
        }
    }

    pub fn sendto(&self, fd: i64, len: usize) -> i64 {
        match self.connected.get(&fd) {
            Some(true) => len as i64,
            Some(false) => -107, // ENOTCONN
            None => -9,
        }
    }

    pub fn recvfrom(&self, fd: i64, len: usize) -> i64 {
        match self.connected.get(&fd) {
            Some(true) => len as i64,
            Some(false) => -107,
            None => -9,
        }
    }
}
