#[derive(Debug, Default)]
pub struct SpinLock {
    locked: bool,
}
impl SpinLock {
    pub fn lock(&mut self) -> bool {
        if self.locked {
            false
        } else {
            self.locked = true;
            true
        }
    }
    pub fn unlock(&mut self) {
        self.locked = false;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lock_unlock() {
        let mut l = SpinLock::default();
        assert!(l.lock());
        assert!(!l.lock());
        l.unlock();
        assert!(l.lock());
    }
}
