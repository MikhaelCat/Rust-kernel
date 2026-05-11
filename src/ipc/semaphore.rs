#[derive(Debug)]
pub struct Semaphore {
    count: isize,
}
impl Semaphore {
    pub fn new(count: isize) -> Self {
        Self { count }
    }
    pub fn up(&mut self) {
        self.count += 1;
    }
    pub fn down(&mut self) -> bool {
        if self.count > 0 {
            self.count -= 1;
            true
        } else {
            false
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn down_up() {
        let mut s = Semaphore::new(1);
        assert!(s.down());
        assert!(!s.down());
        s.up();
        assert!(s.down());
    }
}
