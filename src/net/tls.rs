#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlsState {
    Off,
    On,
}
pub fn enable_tls(sock_open: bool) -> TlsState {
    if sock_open {
        TlsState::On
    } else {
        TlsState::Off
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tls_on_when_open() {
        assert_eq!(enable_tls(true), TlsState::On);
    }
}
