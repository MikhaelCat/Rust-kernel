use std::collections::VecDeque;
#[derive(Debug, Default)]
pub struct RingBuf {
    q: VecDeque<u8>,
    cap: usize,
}
impl RingBuf {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            q: VecDeque::new(),
            cap,
        }
    }
    pub fn push(&mut self, b: u8) -> bool {
        if self.q.len() < self.cap {
            self.q.push_back(b);
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
    fn ring_capacity() {
        let mut r = RingBuf::with_capacity(1);
        assert!(r.push(1));
        assert!(!r.push(2));
    }
}
