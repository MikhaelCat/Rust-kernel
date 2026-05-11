#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetError {
    PayloadTooLarge,
    InvalidMtu,
    DeviceNotFound,
    RouteNotFound,
    SocketNotFound,
    SocketClosed,
    NotBound,
    NotListening,
    NoPendingConnection,
}
