//! TCP/IP Protocol Implementation for Linux Kernel on Rust
//!
//! Реализация протоколов TCP и IP для сетевого стека

use super::error::NetError;

/// Типы IP протоколов
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpProtocol {
    Tcp,      // Transmission Control Protocol
    Udp,      // User Datagram Protocol  
    Icmp,     // Internet Control Message Protocol
    Igmp,     // Internet Group Management Protocol
    Raw,      // Raw sockets
}

/// Порты TCP (0-65535)
pub const MAX_PORT: u16 = 65535;

/// Состояния TCP соединения
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpState {
    Closed,           // No connection
    Listen,           // Listening for connections
    SynSent,          // SYN sent, waiting for SYN+ACK
    SynReceived,      // SYN+ACK received, sending ACK
    Established,      // Connection established
    FinWait1,         // FIN sent, waiting for FIN
    FinWait2,         // Waiting for remote FIN
    CloseWait,        // Remote FIN received, closing locally
    Closing,          // FIN sent, waiting for FIN
    LastAck,          // FIN+ACK sent, waiting for final ACK
    TimeWait,         // Waiting 2MSL before彻底 close
}

impl Default for TcpState {
    fn default() -> Self {
        Self::Closed
    }
}

/// TCP Flags - контрольные биты в TCP header
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TcpFlags(u16);

impl TcpFlags {
    pub const NONE: u16 = 0;
    pub const FIN: u16 = 1 << 0;   // Finish
    pub const SYN: u16 = 1 << 1;   // Synchronize sequence numbers
    pub const RST: u16 = 1 << 2;   // Reset connection
    pub const PSH: u16 = 1 << 3;   // Push data
    pub const ACK: u16 = 1 << 4;   // Acknowledgment
    pub const URG: u16 = 1 << 5;   // Urgent pointer
    pub const ECE: u16 = 1 << 6;   // ECN Echo
    pub const CWR: u16 = 1 << 7;   // Congestion Window Reduced
    
    pub fn empty() -> Self { Self(Self::NONE) }
    
    pub fn with_fin(mut self) -> Self {
        self.0 |= Self::FIN;
        self
    }
    
    pub fn with_syn(mut self) -> Self {
        self.0 |= Self::SYN;
        self
    }
    
    pub fn with_ack(mut self) -> Self {
        self.0 |= Self::ACK;
        self
    }
    
    pub fn with_rst(mut self) -> Self {
        self.0 |= Self::RST;
        self
    }
    
    pub fn has(&self, flag: u16) -> bool {
        self.0 & flag != 0
    }
    
    pub fn is_control(&self) -> bool {
        self.has(Self::FIN) || self.has(Self::SYN) || self.has(Self::RST)
    }
}

/// TCP Header structure
#[derive(Debug, Clone)]
pub struct TcpHeader {
    pub source_port: u16,           // Source port
    pub dest_port: u16,             // Destination port
    pub seq_num: u32,               // Sequence number
    pub ack_num: u32,               // Acknowledgment number
    pub data_offset: u8,            // Data offset in 32-bit words
    pub flags: TcpFlags,            // Control flags
    pub window_size: u16,           // Receive window size
    pub checksum: u16,              // Checksum
    pub urgent_ptr: u16,            // Urgent pointer
}

impl Default for TcpHeader {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl TcpHeader {
    pub fn new(source_port: u16, dest_port: u16) -> Self {
        Self {
            source_port,
            dest_port,
            seq_num: 0,
            ack_num: 0,
            data_offset: 20 / 4, // Minimum header length (20 bytes = 5 * 32-bit)
            flags: TcpFlags::empty(),
            window_size: 65535,    // Max window size
            checksum: 0,
            urgent_ptr: 0,
        }
    }
    
    /// Проверить является ли пакет control packet'ом (SYN/FIN/RST)
    pub fn is_control(&self) -> bool {
        self.flags.is_control()
    }
    
    /// Имеет ли ACK флаг
    pub fn is_ack(&self) -> bool {
        self.flags.has(TcpFlags::ACK)
    }
    
    /// Начинать новое соединение (SYN)
    pub fn syn(&mut self, seq: u32) {
        self.seq_num = seq;
        self.flags = TcpFlags::with_syn();
    }
    
    /// Установить соединение (SYN+ACK)
    pub fn syn_ack(&mut self, seq: u32, ack: u32) {
        self.seq_num = seq;
        self.ack_num = ack;
        self.flags = TcpFlags::with_syn().with_ack();
    }
    
    /// Завершить соединение (FIN)
    pub fn fin(&mut self, seq: u32) {
        self.seq_num = seq;
        self.flags = self.flags.with_fin();
    }
    
    /// Отправить данные
    pub fn send_data(&mut self, seq: u32, ack: u32, len: usize) {
        self.seq_num = seq;
        self.ack_num = ack;
        self.flags = self.flags.with_ack().with_psh();
    }
    
    /// Reset connection
    pub fn reset(&mut self) {
        self.flags = TcpFlags::with_rst();
    }
}

/// Socket Options для настройки сокета
#[derive(Debug, Clone)]
pub struct SocketOptions {
    pub receive_buffer_size: usize,
    pub send_buffer_size: usize,
    pub tcp_no_delay: bool,           // TCP_NODELAY
    pub keep_alive: bool,             // SO_KEEPALIVE
    pub reuse_address: bool,          // SO_REUSEADDR
    pub broadcast: bool,              // SO_BROADCAST
    pub linger: Option<u32>,          // SO_LINGER seconds
}

impl Default for SocketOptions {
    fn default() -> Self {
        Self {
            receive_buffer_size: 65536,   // 64KB default
            send_buffer_size: 65536,
            tcp_no_delay: true,           // Nagle's algorithm off by default
            keep_alive: false,
            reuse_address: false,
            broadcast: false,
            linger: None,
        }
    }
}

/// TCP Контекст соединения
#[derive(Debug, Clone)]
pub struct TcpConnection {
    pub state: TcpState,
    pub local_addr: String,         // Local IP:port
    pub remote_addr: String,        // Remote IP:port
    pub local_port: u16,
    pub remote_port: u16,
    pub send_seq: u32,              // Next send sequence number
    pub recv_seq: u32,              // Next expected receive sequence
    pub send_unacked: u32,          // First unacknowledged byte
    pub receive_window: u32,        // Current receive window
    pub send_buffer: Vec<u8>,       // Outgoing data buffer
    pub receive_buffer: Vec<u8>,    // Incoming data buffer
    pub retransmit_timer: u64,      // Retransmission timeout (ms)
    pub keep_alive_timer: u64,      // Keep-alive timer
}

impl Default for TcpConnection {
    fn default() -> Self {
        Self::new()
    }
}

impl TcpConnection {
    pub fn new() -> Self {
        Self {
            state: TcpState::Closed,
            local_addr: "0.0.0.0:0".to_string(),
            remote_addr: "".to_string(),
            local_port: 0,
            remote_port: 0,
            send_seq: 0,
            recv_seq: 0,
            send_unacked: 0,
            receive_window: 65535,
            send_buffer: Vec::new(),
            receive_buffer: Vec::new(),
            retransmit_timer: 0,
            keep_alive_timer: 0,
        }
    }
    
    /// Start listening on local port
    pub fn start_listen(&mut self, local_port: u16) {
        self.local_port = local_port;
        self.state = TcpState::Listen;
    }
    
    /// Initiate connection (send SYN)
    pub fn connect(&mut self, remote_addr: String, remote_port: u16) {
        self.remote_addr = remote_addr;
        self.remote_port = remote_port;
        self.send_seq = self.random_sequence_number();
        self.state = TcpState::SynSent;
    }
    
    /// Accept incoming connection
    pub fn accept(&mut self) {
        self.state = TcpState::Established;
    }
    
    /// Send data over connection
    pub fn send(&mut self, data: &[u8]) -> Result<usize, NetError> {
        if self.state != TcpState::Established {
            return Err(NetError::InvalidState);
        }
        
        // Buffer the data
        let len = data.len();
        self.send_buffer.extend_from_slice(data);
        
        Ok(len)
    }
    
    /// Receive data from connection
    pub fn receive(&mut self, buf: &mut [u8]) -> Result<usize, NetError> {
        if self.state != TcpState::Established && 
           self.state != TcpState::CloseWait {
            return Err(NetError::InvalidState);
        }
        
        // Copy from receive buffer
        let to_copy = buf.len().min(self.receive_buffer.len());
        if to_copy > 0 {
            buf[..to_copy].copy_from_slice(&self.receive_buffer[..to_copy]);
            self.receive_buffer.drain(..to_copy);
        }
        
        Ok(to_copy)
    }
    
    /// Close connection gracefully (send FIN)
    pub fn close(&mut self) {
        if self.state == TcpState::Established {
            self.state = TcpState::FinWait1;
        } else if self.state == TcpState::CloseWait {
            self.state = TcpState::LastAck;
        }
    }
    
    /// Reset connection abruptly (send RST)
    pub fn reset(&mut self) {
        self.state = TcpState::Closed;
        self.recv_seq = 0;
        self.send_seq = 0;
    }
    
    /// Generate random initial sequence number (ISN)
    fn random_sequence_number(&self) -> u32 {
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u32;
        now ^ (self.local_port as u32) ^ (self.remote_port as u32)
    }
    
    /// Check if connection is established
    pub fn is_established(&self) -> bool {
        matches!(self.state, TcpState::Established | TcpState::SynReceived)
    }
    
    /// Check if connection is closing
    pub fn is_closing(&self) -> bool {
        matches!(self.state, 
            TcpState::FinWait1 | TcpState::FinWait2 | 
            TcpState::Closing | TcpState::LastAck | 
            TcpState::TimeWait | TcpState::CloseWait)
    }
    
    /// Get current window size
    pub fn window_size(&self) -> u32 {
        // Simplified: always report full window after ACK
        self.receive_window.max(1024) // At least 1KB
    }
}

/// TCP Socket - точка network communication
#[derive(Debug, Clone)]
pub struct TcpSocket {
    pub socket_id: u32,              // Unique socket identifier
    pub fd: i32,                     // File descriptor
    pub local_addr: SocketAddrV4,    // Local address
    pub remote_addr: Option<SocketAddrV4>, // Remote address (if connected)
    pub state: TcpState,
    pub connection: Option<TcpConnection>,
    pub options: SocketOptions,
    pub backlog: usize,              // Listen queue size
    pub accept_queue: Vec<TcpSocket>, // Accepted connections
}

impl Default for TcpSocket {
    fn default() -> Self {
        Self::new()
    }
}

impl TcpSocket {
    pub fn new() -> Self {
        Self {
            socket_id: 0,
            fd: -1,
            local_addr: SocketAddrV4::default(),
            remote_addr: None,
            state: TcpState::Closed,
            connection: None,
            options: SocketOptions::default(),
            backlog: 128,                 // Typical Linux default
            accept_queue: Vec::new(),
        }
    }
    
    /// Bind socket to local address and port
    pub fn bind(&mut self, addr: SocketAddrV4) -> Result<(), NetError> {
        if addr.port > MAX_PORT {
            return Err(NetError::InvalidParameter);
        }
        
        self.local_addr = addr;
        self.state = TcpState::Closed;
        
        Ok(())
    }
    
    /// Start listening for connections
    pub fn listen(&mut self, backlog: usize) -> Result<(), NetError> {
        if self.local_addr.port == 0 {
            return Err(NetError::InvalidAddress);
        }
        
        self.backlog = backlog.min(1024); // Cap at 1024
        self.state = TcpState::Listen;
        
        Ok(())
    }
    
    /// Initiate connection to remote host
    pub fn connect(&mut self, remote: SocketAddrV4) -> Result<(), NetError> {
        if self.local_addr.port == 0 {
            return Err(NetError::InvalidAddress);
        }
        
        self.remote_addr = Some(remote);
        if let Some(ref mut conn) = self.connection {
            conn.connect(format!("{}:{}", remote.ip, remote.port), remote.port);
        }
        
        Ok(())
    }
    
    /// Accept incoming connection
    pub fn accept(&mut self) -> Result<&mut TcpSocket, NetError> {
        if self.state != TcpState::Listen {
            return Err(NetError::InvalidState);
        }
        
        if !self.accept_queue.is_empty() {
            let accepted = self.accept_queue.remove(0);
            return Ok(accepted);
        }
        
        Err(NetError::WouldBlock) // No pending connections
    }
    
    /// Send data through socket
    pub fn send(&mut self, data: &[u8]) -> Result<usize, NetError> {
        if self.state != TcpState::Established {
            return Err(NetError::InvalidState);
        }
        
        if let Some(ref mut conn) = self.connection {
            conn.send(data)
        } else {
            Err(NetError::NoSuchProcess)
        }
    }
    
    /// Receive data from socket
    pub fn receive(&mut self, buf: &mut [u8]) -> Result<usize, NetError> {
        if let Some(ref mut conn) = self.connection {
            conn.receive(buf)
        } else {
            Err(NetError::InvalidState)
        }
    }
    
    /// Close socket
    pub fn close(&mut self) -> Result<(), NetError> {
        if self.state == TcpState::Established {
            if let Some(ref mut conn) = self.connection {
                conn.close();
            }
        }
        
        self.state = TcpState::Closed;
        Ok(())
    }
    
    /// Set socket option
    pub fn set_option(&mut self, name: &str, value: bool) {
        match name {
            "tcp_nodelay" => self.options.tcp_no_delay = value,
            "keep_alive" => self.options.keep_alive = value,
            "reuse_address" => self.options.reuse_address = value,
            "broadcast" => self.options.broadcast = value,
            _ => {}
        }
    }
}

/// IPv4 Socket Address
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SocketAddrV4 {
    pub ip: u32,                    // IPv4 address (in network byte order)
    pub port: u16,                  // Port number
}

impl Default for SocketAddrV4 {
    fn default() -> Self {
        Self::new(std::net::Ipv4Addr::UNSPECIFIED, 0)
    }
}

impl SocketAddrV4 {
    pub fn new(ip: std::net::Ipv4Addr, port: u16) -> Self {
        // Convert to network byte order
        let ip_bytes = ip.octets();
        let ip_network = u32::from_be_bytes(ip_bytes);
        
        Self {
            ip: ip_network,
            port,
        }
    }
    
    /// Create from string like "192.168.1.1:80"
    pub fn parse(addr_str: &str) -> Result<Self, NetError> {
        let parts: Vec<&str> = addr_str.split(':').collect();
        
        if parts.len() != 2 {
            return Err(NetError::InvalidAddress);
        }
        
        let ip_str = parts[0];
        let port_str = parts[1];
        
        let ip: u32 = ip_str.parse()
            .map_err(|_| NetError::InvalidAddress)?
            .to_le(); // Network byte order
        
        let port = port_str.parse()
            .map_err(|_| NetError::InvalidPort)?
            .min(MAX_PORT);
        
        Ok(Self { ip, port })
    }
    
    /// Convert to standard format
    pub fn to_standard(self) -> std::net::SocketAddrV4 {
        let ip_bytes = self.ip.to_le_bytes();
        let ip = std::net::Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
        
        std::net::SocketAddrV4::new(ip, self.port)
    }
    
    /// Check if localhost
    pub fn is_loopback(&self) -> bool {
        let ip_bytes = self.ip.to_le_bytes();
        ip_bytes[0] == 127
    }
}

/// ICMP (Internet Control Message Protocol) implementation
#[derive(Debug, Clone)]
pub struct IcmpMessage {
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub checksum: u16,
    pub rest_of_header: [u8; 4],
    pub data: Vec<u8>,
}

impl IcmpMessage {
    pub fn echo_request(identifier: u16) -> Self {
        Self {
            icmp_type: 8,     // Echo Request
            icmp_code: 0,
            checksum: 0,
            rest_of_header: [(identifier >> 8) as u8, (identifier & 0xff) as u8, 0, 0],
            data: Vec::new(),
        }
    }
    
    pub fn echo_reply(data: &[u8]) -> Self {
        Self {
            icmp_type: 0,     // Echo Reply
            icmp_code: 0,
            checksum: 0,
            rest_of_header: [0, 0, 0, 0],
            data: data.to_vec(),
        }
    }
}

/// Network Stack Manager - основной менеджер сети
#[derive(Debug)]
pub struct NetworkStack {
    pub tcp_sockets: HashMap<u32, TcpSocket>,  // All open TCP sockets
    pub next_socket_id: AtomicU32,
    pub active_connections: u32,
    pub packets_received: AtomicU64,
    pub packets_sent: AtomicU64,
    pub errors: AtomicU64,
}

impl Default for NetworkStack {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkStack {
    pub fn new() -> Self {
        Self {
            tcp_sockets: HashMap::new(),
            next_socket_id: AtomicU32::new(1),
            active_connections: 0,
            packets_received: AtomicU64::new(0),
            packets_sent: AtomicU64::new(0),
            errors: AtomicU64::new(0),
        }
    }
    
    /// Create a new TCP socket
    pub fn create_socket(&mut self, listen_port: u16) -> u32 {
        let id = self.next_socket_id.fetch_add(1, Ordering::SeqCst);
        
        let mut socket = TcpSocket::new();
        socket.socket_id = id;
        
        if listen_port != 0 {
            let addr = SocketAddrV4::new(std::net::Ipv4Addr::UNSPECIFIED, listen_port);
            socket.bind(addr).ok();
        }
        
        self.tcp_sockets.insert(id, socket);
        id
    }
    
    /// Get socket by ID
    pub fn get_socket(&self, id: u32) -> Option<&TcpSocket> {
        self.tcp_sockets.get(&id)
    }
    
    /// Get mutable socket reference
    pub fn get_socket_mut(&mut self, id: u32) -> Option<&mut TcpSocket> {
        self.tcp_sockets.get_mut(&id)
    }
    
    /// Close socket
    pub fn close_socket(&mut self, id: u32) -> Result<bool, NetError> {
        if let Some(mut socket) = self.tcp_sockets.remove(&id) {
            socket.close()?;
            return Ok(true);
        }
        
        Err(NetError::InvalidAddress)
    }
    
    /// Get statistics
    pub fn stats(&self) -> NetworkStats {
        NetworkStats {
            active_tcp_sockets: self.tcp_sockets.len() as u64,
            active_connections: self.active_connections,
            packets_received: self.packets_received.load(Ordering::SeqCst),
            packets_sent: self.packets_sent.load(Ordering::SeqCst),
            total_errors: self.errors.load(Ordering::SeqCst),
        }
    }
}

/// Статистика сети
#[derive(Debug, Clone)]
pub struct NetworkStats {
    pub active_tcp_sockets: u64,
    pub active_connections: u32,
    pub packets_received: u64,
    pub packets_sent: u64,
    pub total_errors: u64,
}

impl NetworkStats {
    pub fn throughput(&self) -> f64 {
        (self.packets_received as f64 + self.packets_sent as f64) / 2.0
    }
}

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tcp_flags() {
        let flags = TcpFlags::with_syn().with_ack();
        assert!(flags.has(TcpFlags::SYN));
        assert!(flags.has(TcpFlags::ACK));
        assert!(!flags.has(TcpFlags::FIN));
    }
    
    #[test]
    fn test_tcp_header_creation() {
        let mut hdr = TcpHeader::new(12345, 80);
        hdr.syn(1000);
        
        assert_eq!(hdr.source_port, 12345);
        assert_eq!(hdr.dest_port, 80);
        assert!(hdr.flags.has(TcpFlags::SYN));
    }
    
    #[test]
    fn test_socketaddr_parse() {
        let addr = SocketAddrV4::parse("192.168.1.1:80").unwrap();
        assert_eq!(addr.port, 80);
    }
    
    #[test]
    fn test_tcp_socket_operations() {
        let mut sock = TcpSocket::new();
        
        // Bind and listen
        let addr = SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 8080);
        sock.bind(addr).unwrap();
        sock.listen(10).unwrap();
        
        assert_eq!(sock.state, TcpState::Listen);
    }
    
    #[test]
    fn test_network_stack_create_socket() {
        let mut stack = NetworkStack::new();
        
        let sock_id = stack.create_socket(80);
        assert_eq!(stack.stats().active_tcp_sockets, 1);
        
        stack.close_socket(sock_id).unwrap();
        assert_eq!(stack.stats().active_tcp_sockets, 0);
    }
}
