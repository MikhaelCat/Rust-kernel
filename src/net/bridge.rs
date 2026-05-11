#[derive(Debug, Default)]
pub struct Bridge {
    ports: u32,
}
impl Bridge {
    pub fn add_port(&mut self) {
        self.ports += 1;
    }
    pub fn ports(&self) -> u32 {
        self.ports
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_bridge_port() {
        let mut b = Bridge::default();
        b.add_port();
        assert_eq!(b.ports(), 1);
    }
}
