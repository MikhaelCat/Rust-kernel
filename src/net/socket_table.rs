use std::collections::BTreeMap;

use super::socket::Socket;

#[derive(Debug, Default)]
pub struct SocketTable {
    next_id: u64,
    sockets: BTreeMap<u64, Socket>,
}

impl SocketTable {
    pub fn open(&mut self) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.sockets.insert(id, Socket::new(id));
        id
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut Socket> {
        self.sockets.get_mut(&id)
    }

    pub fn get(&self, id: u64) -> Option<&Socket> {
        self.sockets.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Socket> {
        self.sockets.values()
    }

    pub fn contains(&self, id: u64) -> bool {
        self.sockets.contains_key(&id)
    }
}
