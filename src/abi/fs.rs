use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct FileTable {
    next_fd: i64,
    files: BTreeMap<i64, String>,
}

impl FileTable {
    pub fn new() -> Self {
        Self {
            next_fd: 3,
            files: BTreeMap::new(),
        }
    }

    pub fn open(&mut self, path: &str) -> i64 {
        let fd = self.next_fd;
        self.next_fd += 1;
        self.files.insert(fd, path.to_string());
        fd
    }

    pub fn close(&mut self, fd: i64) -> i64 {
        if self.files.remove(&fd).is_some() {
            0
        } else {
            -9
        }
    }

    pub fn write(&self, fd: i64, buf_len: usize) -> i64 {
        if self.files.contains_key(&fd) {
            buf_len as i64
        } else {
            -9
        }
    }

    pub fn read(&self, fd: i64, want: usize) -> i64 {
        if self.files.contains_key(&fd) {
            want as i64
        } else {
            -9
        }
    }
}
