#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessAbi {
    pub pid: i64,
}

impl ProcessAbi {
    pub fn new(pid: i64) -> Self {
        Self { pid }
    }

    pub fn getpid(&self) -> i64 {
        self.pid
    }
}
