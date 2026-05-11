#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TcpState {
    Closed,
    SynSent,
    Established,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TcpSocket {
    pub state: TcpState,
}
impl TcpSocket {
    pub fn connect() -> Self {
        Self {
            state: TcpState::Established,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tcp_connect() {
        assert_eq!(TcpSocket::connect().state, TcpState::Established);
    }
}
