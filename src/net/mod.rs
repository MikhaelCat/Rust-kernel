pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;

pub mod device;
pub mod error;
pub mod manager;
pub mod packet;
pub mod route;
pub mod socket;
pub mod socket_table;

pub use device::NetDevice;
pub use error::NetError;
pub use manager::NetworkStack;
pub use packet::{Ipv4Packet, Ipv6Packet};
pub use route::Route;
pub use socket::Socket;
pub use socket_table::SocketTable;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_encode_still_works() {
        let pkt = Ipv6Packet {
            src: [0; 16],
            dst: [1; 16],
            next_header: 17,
            payload: vec![1, 2, 3],
        };
        let raw = pkt.encode().expect("encode failed");
        assert_eq!(raw[0] >> 4, 6);
    }
}
pub mod bridge;
pub mod conntrack;
pub mod firewall;
pub mod gro;
pub mod icmpv6;
pub mod napi;
pub mod neigh;
pub mod qdisc;
pub mod rps;
pub mod tcp;
pub mod tls;
pub mod udp;
pub mod xdp;

pub mod status;
pub mod system;
pub use system::NetSystem;
pub mod health;

pub mod system2;

pub mod offload;

pub mod ethtool;

pub mod components;

pub mod profile;

pub mod fault;

pub mod socket_state;

pub mod listener;
pub mod session;

pub mod conn_matrix;

pub mod errno;
