#[derive(Debug, Default)]
pub struct Uart {
    tx: usize,
}
impl Uart {
    pub fn send(&mut self, _b: u8) {
        self.tx += 1;
    }
    pub fn tx_count(&self) -> usize {
        self.tx
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uart_send() {
        let mut u = Uart::default();
        u.send(1);
        assert_eq!(u.tx_count(), 1);
    }
}
