#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtNet {
    pub mac: [u8; 6],
    pub link_up: bool,
}

impl VirtNet {
    pub fn new(mac: [u8; 6]) -> Self {
        Self {
            mac,
            link_up: false,
        }
    }

    pub fn up(&mut self) {
        self.link_up = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_up() {
        let mut n = VirtNet::new([0, 1, 2, 3, 4, 5]);
        n.up();
        assert!(n.link_up);
    }
}
