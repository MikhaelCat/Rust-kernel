#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub len: u16,
}
impl UdpHeader {
    pub fn new(src: u16, dst: u16, payload_len: u16) -> Self {
        Self {
            src_port: src,
            dst_port: dst,
            len: payload_len + 8,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn udp_len() {
        assert_eq!(UdpHeader::new(1, 2, 4).len, 12);
    }
}
