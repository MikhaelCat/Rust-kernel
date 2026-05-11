#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SuspendState {
    Active,
    Suspended,
}
#[derive(Debug)]
pub struct SuspendManager {
    state: SuspendState,
}
impl SuspendManager {
    pub fn new() -> Self {
        Self {
            state: SuspendState::Active,
        }
    }
    pub fn suspend(&mut self) {
        self.state = SuspendState::Suspended;
    }
    pub fn resume(&mut self) {
        self.state = SuspendState::Active;
    }
    pub fn state(&self) -> SuspendState {
        self.state
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn suspend_resume() {
        let mut s = SuspendManager::new();
        s.suspend();
        s.resume();
        assert_eq!(s.state(), SuspendState::Active);
    }
}
