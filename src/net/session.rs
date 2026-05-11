#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Init,
    Established,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetSession {
    pub state: SessionState,
}

impl NetSession {
    pub fn new() -> Self {
        Self {
            state: SessionState::Init,
        }
    }

    pub fn establish(&mut self) {
        self.state = SessionState::Established;
    }

    pub fn close(&mut self) {
        self.state = SessionState::Closed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_flow() {
        let mut s = NetSession::new();
        s.establish();
        s.close();
        assert_eq!(s.state, SessionState::Closed);
    }
}
