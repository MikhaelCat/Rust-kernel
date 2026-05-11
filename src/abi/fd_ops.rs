use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Default)]
pub struct FdOps {
    next_fd: i64,
    aliases: BTreeMap<i64, i64>,
    pipes: BTreeMap<i64, VecDeque<u8>>,
}

impl FdOps {
    pub fn new() -> Self {
        Self {
            next_fd: 300,
            aliases: BTreeMap::new(),
            pipes: BTreeMap::new(),
        }
    }

    pub fn alloc_fd(&mut self) -> i64 {
        let fd = self.next_fd;
        self.next_fd += 1;
        fd
    }

    pub fn dup(&mut self, fd: i64) -> i64 {
        if fd < 0 {
            return -9;
        }
        let new_fd = self.alloc_fd();
        self.aliases.insert(new_fd, fd);
        new_fd
    }

    pub fn dup2(&mut self, oldfd: i64, newfd: i64) -> i64 {
        if oldfd < 0 || newfd < 0 {
            return -9;
        }
        self.aliases.insert(newfd, oldfd);
        newfd
    }

    pub fn pipe(&mut self) -> (i64, i64) {
        let rfd = self.alloc_fd();
        let wfd = self.alloc_fd();
        self.pipes.insert(rfd, VecDeque::new());
        self.aliases.insert(wfd, rfd);
        (rfd, wfd)
    }

    pub fn pipe_write(&mut self, wfd: i64, b: u8) -> i64 {
        let Some(&rfd) = self.aliases.get(&wfd) else {
            return -9;
        };
        let Some(q) = self.pipes.get_mut(&rfd) else {
            return -9;
        };
        q.push_back(b);
        1
    }

    pub fn pipe_read(&mut self, rfd: i64) -> i64 {
        let Some(q) = self.pipes.get_mut(&rfd) else {
            return -9;
        };
        q.pop_front().map(|b| b as i64).unwrap_or(-11)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dup_and_pipe_flow() {
        let mut f = FdOps::new();
        let fd = f.alloc_fd();
        let d = f.dup(fd);
        assert!(d >= 300);
        assert_eq!(f.dup2(fd, 10), 10);

        let (r, w) = f.pipe();
        assert_eq!(f.pipe_write(w, b'A'), 1);
        assert_eq!(f.pipe_read(r), b'A' as i64);
    }
}
