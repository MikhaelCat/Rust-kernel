#[derive(Debug, Default)]
pub struct DriverMailbox {
    tx: u64,
}
impl DriverMailbox {
    pub fn send(&mut self) {
        self.tx += 1;
    }
    pub fn tx(&self) -> u64 {
        self.tx
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn send_msg() {
        let mut m = DriverMailbox::default();
        m.send();
        assert_eq!(m.tx(), 1);
    }
}
