#[derive(Debug, Default)]
pub struct Lockdep {
    locks: u64,
}
impl Lockdep {
    pub fn acquire(&mut self) {
        self.locks += 1;
    }
    pub fn locks(&self) -> u64 {
        self.locks
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn acquire_lock() {
        let mut l = Lockdep::default();
        l.acquire();
        assert_eq!(l.locks(), 1);
    }
}
