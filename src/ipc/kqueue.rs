#[derive(Debug, Default)]
pub struct KQueue {
    events: u64,
}
impl KQueue {
    pub fn push_event(&mut self) {
        self.events += 1;
    }
    pub fn events(&self) -> u64 {
        self.events
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_event() {
        let mut k = KQueue::default();
        k.push_event();
        assert_eq!(k.events(), 1);
    }
}
