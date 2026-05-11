#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Probed,
    Bound,
    Suspended,
    Removed,
}

#[derive(Debug, Default)]
pub struct DriverLifecycle {
    state: Option<DriverState>,
}

impl DriverLifecycle {
    pub fn probe(&mut self) {
        self.state = Some(DriverState::Probed);
    }

    pub fn bind(&mut self) {
        self.state = Some(DriverState::Bound);
    }

    pub fn suspend(&mut self) {
        self.state = Some(DriverState::Suspended);
    }

    pub fn remove(&mut self) {
        self.state = Some(DriverState::Removed);
    }

    pub fn state(&self) -> Option<DriverState> {
        self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_flow() {
        let mut l = DriverLifecycle::default();
        l.probe();
        l.bind();
        l.suspend();
        l.remove();
        assert_eq!(l.state(), Some(DriverState::Removed));
    }
}
