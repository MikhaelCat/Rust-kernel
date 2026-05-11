#[derive(Debug, Default)]
pub struct Futex {
    waiters: u32,
}
impl Futex {
    pub fn wait(&mut self) {
        self.waiters += 1;
    }
    pub fn wake_one(&mut self) {
        if self.waiters > 0 {
            self.waiters -= 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn futex_wait_wake() {
        let mut f = Futex::default();
        f.wait();
        f.wake_one();
        assert_eq!(f.waiters, 0);
    }
}
