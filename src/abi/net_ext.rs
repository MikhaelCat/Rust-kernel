use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Created,
    Bound,
    Listening,
    Connected,
}

#[derive(Debug, Default)]
pub struct NetExt {
    next_fd: i64,
    states: BTreeMap<i64, State>,
}

impl NetExt {
    pub fn new() -> Self {
        Self {
            next_fd: 200,
            states: BTreeMap::new(),
        }
    }

    pub fn socket(&mut self) -> i64 {
        let fd = self.next_fd;
        self.next_fd += 1;
        self.states.insert(fd, State::Created);
        fd
    }

    pub fn bind(&mut self, fd: i64) -> i64 {
        match self.states.get_mut(&fd) {
            Some(s @ State::Created) => {
                *s = State::Bound;
                0
            }
            Some(_) => -22,
            None => -9,
        }
    }

    pub fn listen(&mut self, fd: i64) -> i64 {
        match self.states.get_mut(&fd) {
            Some(s @ State::Bound) => {
                *s = State::Listening;
                0
            }
            Some(_) => -22,
            None => -9,
        }
    }

    pub fn accept(&mut self, fd: i64) -> i64 {
        match self.states.get(&fd) {
            Some(State::Listening) => {
                let nfd = self.next_fd;
                self.next_fd += 1;
                self.states.insert(nfd, State::Connected);
                nfd
            }
            Some(_) => -22,
            None => -9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_listen_accept() {
        let mut n = NetExt::new();
        let fd = n.socket();
        assert_eq!(n.bind(fd), 0);
        assert_eq!(n.listen(fd), 0);
        assert!(n.accept(fd) >= 200);
    }
}
