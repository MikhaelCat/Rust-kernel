use std::collections::VecDeque;
#[derive(Debug, Default)]
pub struct Mailbox {
    q: VecDeque<u64>,
}
impl Mailbox {
    pub fn post(&mut self, v: u64) {
        self.q.push_back(v);
    }
    pub fn recv(&mut self) -> Option<u64> {
        self.q.pop_front()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn post_recv() {
        let mut m = Mailbox::default();
        m.post(7);
        assert_eq!(m.recv(), Some(7));
    }
}
