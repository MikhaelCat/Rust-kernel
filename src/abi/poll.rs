#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PollFd {
    pub fd: i64,
    pub readable: bool,
    pub writable: bool,
}

pub fn poll_like(fds: &[PollFd], _timeout_ms: i32) -> i64 {
    fds.iter().filter(|f| f.readable || f.writable).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_counts_ready() {
        let f = [
            PollFd {
                fd: 1,
                readable: true,
                writable: false,
            },
            PollFd {
                fd: 2,
                readable: false,
                writable: false,
            },
        ];
        assert_eq!(poll_like(&f, 0), 1);
    }
}
