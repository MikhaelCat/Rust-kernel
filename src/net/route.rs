#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    pub prefix: [u8; 16],
    pub plen: u8,
    pub dev: String,
}

impl Route {
    pub fn default_v6(dev: &str) -> Self {
        Self {
            prefix: [0; 16],
            plen: 0,
            dev: dev.to_string(),
        }
    }
}
