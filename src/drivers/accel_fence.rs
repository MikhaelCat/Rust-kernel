#[derive(Debug, Default)]
pub struct AccelFence {
    done: bool,
}
impl AccelFence {
    pub fn signal(&mut self) {
        self.done = true;
    }
    pub fn is_done(&self) -> bool {
        self.done
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signal_fence() {
        let mut f = AccelFence::default();
        f.signal();
        assert!(f.is_done());
    }
}
