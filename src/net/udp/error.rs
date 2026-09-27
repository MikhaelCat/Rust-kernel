//! UDP error handling for Linux kernel networking

/// UDP-specific error codes
#[derive(Debug, Clone, Copy)]
pub enum UdpError {
    InvalidChecksum,
    BadDestination,
    SocketClosed,
    BufferOverflow,
    ProtocolError,
}

impl std::fmt::Display for UdpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UdpError::InvalidChecksum => write!(f, "Invalid UDP checksum"),
            UdpError::BadDestination => write!(f, "Bad destination address"),
            UdpError::SocketClosed => write!(f, "Socket closed"),
            UdpError::BufferOverflow => write!(f, "Buffer overflow"),
            UdpError::ProtocolError => write!(f, "UDP protocol error"),
        }
    }
}

impl std::error::Error for UdpError {}
