use super::error::NetError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv4Packet {
    pub src: [u8; 4],
    pub dst: [u8; 4],
    pub protocol: u8,
    pub payload: Vec<u8>,
}

impl Ipv4Packet {
    pub fn encode(&self) -> Result<Vec<u8>, NetError> {
        if self.payload.len() > (u16::MAX as usize).saturating_sub(20) {
            return Err(NetError::PayloadTooLarge);
        }
        let total_len = 20 + self.payload.len();
        let mut out = vec![0u8; total_len];
        out[0] = 0x45;
        out[2] = ((total_len >> 8) & 0xff) as u8;
        out[3] = (total_len & 0xff) as u8;
        out[8] = 64;
        out[9] = self.protocol;
        out[12..16].copy_from_slice(&self.src);
        out[16..20].copy_from_slice(&self.dst);
        out[20..].copy_from_slice(&self.payload);
        Ok(out)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ipv6Packet {
    pub src: [u8; 16],
    pub dst: [u8; 16],
    pub next_header: u8,
    pub payload: Vec<u8>,
}

impl Ipv6Packet {
    pub fn encode(&self) -> Result<Vec<u8>, NetError> {
        if self.payload.len() > u16::MAX as usize {
            return Err(NetError::PayloadTooLarge);
        }

        let payload_len = self.payload.len() as u16;
        let mut out = vec![0u8; 40 + self.payload.len()];
        out[0] = 0x60;
        out[4] = (payload_len >> 8) as u8;
        out[5] = payload_len as u8;
        out[6] = self.next_header;
        out[7] = 64;
        out[8..24].copy_from_slice(&self.src);
        out[24..40].copy_from_slice(&self.dst);
        out[40..].copy_from_slice(&self.payload);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_encode_smoke() {
        let p = Ipv4Packet {
            src: [10, 0, 0, 1],
            dst: [10, 0, 0, 2],
            protocol: 17,
            payload: vec![1, 2, 3, 4],
        };
        let raw = p.encode().expect("encode");
        assert_eq!(raw[0] >> 4, 4);
        assert_eq!(raw.len(), 24);
    }
}
