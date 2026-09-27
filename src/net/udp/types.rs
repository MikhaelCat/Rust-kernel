//! UDP types and structures for Linux kernel networking

use std::net::{Ipv4Addr, Ipv6Addr};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// UDP socket state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UdpSocketState {
    Unbound,
    Bound,
    Connected,
    Listening,
    Closed,
}

/// UDP packet header structure
#[derive(Debug, Clone)]
pub struct UdpHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub length: u16,
    pub checksum: u16,
}

impl Default for UdpHeader {
    fn default() -> Self {
        Self {
            source_port: 0,
            destination_port: 0,
            length: 0,
            checksum: 0,
        }
    }
}

/// UDP socket statistics
#[derive(Debug)]
pub struct UdpStats {
    pub rx_packets: AtomicUsize,
    pub tx_packets: AtomicUsize,
    pub rx_errors: AtomicUsize,
    pub tx_errors: AtomicUsize,
    pub rx_bytes: AtomicUsize,
    pub tx_bytes: AtomicUsize,
    pub listen_overflows: AtomicUsize,
    pub listen_drops: AtomicUsize,
}

impl Default for UdpStats {
    fn default() -> Self {
        Self {
            rx_packets: AtomicUsize::new(0),
            tx_packets: AtomicUsize::new(0),
            rx_errors: AtomicUsize::new(0),
            tx_errors: AtomicUsize::new(0),
            rx_bytes: AtomicUsize::new(0),
            tx_bytes: AtomicUsize::new(0),
            listen_overflows: AtomicUsize::new(0),
            listen_drops: AtomicUsize::new(0),
        }
    }
}

/// UDP protocol configuration
#[derive(Debug)]
pub struct UdpProtocolConfig {
    pub udp_checksum: bool,
    pub udp_timestamps: bool,
    pub udp_ecn: bool,
    pub udp_lro: bool,
}

impl Default for UdpProtocolConfig {
    fn default() -> Self {
        Self {
            udp_checksum: true,
            udp_timestamps: false,
            udp_ecn: true,
            udp_lro: false,
        }
    }
}
