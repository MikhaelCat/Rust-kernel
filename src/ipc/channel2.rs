#[derive(Debug, Default)]
pub struct CounterChannel {
    sends: u64,
}

impl CounterChannel {
    pub fn send(&mut self) {
        self.sends += 1;
    }

    pub fn sends(&self) -> u64 {
        self.sends
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_count() {
        let mut c = CounterChannel::default();
        c.send();
        c.send();
        assert_eq!(c.sends(), 2);
    }
}
