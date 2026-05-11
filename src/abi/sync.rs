#[derive(Debug, Default)]
pub struct FutexTable {
    wakes: u64,
}

impl FutexTable {
    pub fn futex_wait(&self) -> i64 {
        0
    }

    pub fn futex_wake(&mut self, n: i64) -> i64 {
        if n <= 0 {
            return 0;
        }
        self.wakes += n as u64;
        n
    }
}
