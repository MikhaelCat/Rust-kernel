use super::error::NetError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetDevice {
    pub name: String,
    pub mtu: usize,
    pub up: bool,
}

impl NetDevice {
    pub fn new(name: &str, mtu: usize) -> Result<Self, NetError> {
        if mtu < 576 {
            return Err(NetError::InvalidMtu);
        }
        Ok(Self {
            name: name.to_string(),
            mtu,
            up: true,
        })
    }
}
