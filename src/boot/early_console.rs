#[derive(Debug, Default)]
pub struct EarlyConsole {
    bytes_written: usize,
}

impl EarlyConsole {
    pub fn write(&mut self, s: &str) {
        self.bytes_written += s.len();
    }

    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn writes_bytes() {
        let mut c = EarlyConsole::default();
        c.write("boot");
        assert_eq!(c.bytes_written(), 4);
    }
}
