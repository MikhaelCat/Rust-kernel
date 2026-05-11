#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    pub fd: i64,
    pub path: String,
    pub offset: usize,
}

impl File {
    pub fn new(fd: i64, path: &str) -> Self {
        Self {
            fd,
            path: path.to_string(),
            offset: 0,
        }
    }

    pub fn seek(&mut self, off: usize) {
        self.offset = off;
    }
}
