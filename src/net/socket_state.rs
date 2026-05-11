#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SockState {
    Created,
    Bound,
    Connected,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SockFsm {
    pub state: SockState,
}

impl SockFsm {
    pub fn new() -> Self {
        Self {
            state: SockState::Created,
        }
    }

    pub fn bind(&mut self) -> bool {
        if self.state == SockState::Created {
            self.state = SockState::Bound;
            true
        } else {
            false
        }
    }

    pub fn connect(&mut self) -> bool {
        if self.state == SockState::Bound || self.state == SockState::Created {
            self.state = SockState::Connected;
            true
        } else {
            false
        }
    }

    pub fn close(&mut self) {
        self.state = SockState::Closed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn socket_fsm_flow() {
        let mut s = SockFsm::new();
        assert!(s.bind());
        assert!(s.connect());
        s.close();
        assert_eq!(s.state, SockState::Closed);
    }
}
