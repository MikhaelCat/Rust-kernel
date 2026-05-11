#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmState {
    Created,
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmLifecycle {
    pub id: u64,
    pub state: VmState,
}

impl VmLifecycle {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: VmState::Created,
        }
    }

    pub fn start(&mut self) -> bool {
        match self.state {
            VmState::Created | VmState::Stopped => {
                self.state = VmState::Running;
                true
            }
            VmState::Running | VmState::Paused => false,
        }
    }

    pub fn pause(&mut self) -> bool {
        if self.state == VmState::Running {
            self.state = VmState::Paused;
            true
        } else {
            false
        }
    }

    pub fn resume(&mut self) -> bool {
        if self.state == VmState::Paused {
            self.state = VmState::Running;
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self) -> bool {
        match self.state {
            VmState::Stopped | VmState::Created => false,
            VmState::Running | VmState::Paused => {
                self.state = VmState::Stopped;
                true
            }
        }
    }

    pub fn reset(&mut self) -> bool {
        if self.state == VmState::Running {
            return false;
        }
        self.state = VmState::Created;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_states() {
        let mut v = VmLifecycle::new(1);
        assert!(v.start());
        assert!(v.pause());
        assert!(v.resume());
        assert!(v.stop());
        assert_eq!(v.state, VmState::Stopped);
    }

    #[test]
    fn invalid_transitions_blocked() {
        let mut v = VmLifecycle::new(2);
        assert!(!v.pause());
        assert!(!v.resume());
        assert!(!v.stop());
        assert!(v.start());
        assert!(!v.start());
    }

    #[test]
    fn reset_requires_non_running() {
        let mut v = VmLifecycle::new(3);
        assert!(v.reset());
        assert!(v.start());
        assert!(!v.reset());
        assert!(v.stop());
        assert!(v.reset());
        assert_eq!(v.state, VmState::Created);
    }
}
