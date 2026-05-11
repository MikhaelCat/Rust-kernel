#[derive(Debug, Default)]
pub struct NotifierChain {
    events: u64,
}
impl NotifierChain {
    pub fn notify(&mut self) {
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
    fn notify_once() {
        let mut n = NotifierChain::default();
        n.notify();
        assert_eq!(n.events(), 1);
    }
}
