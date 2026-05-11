#[derive(Debug, Default)]
pub struct Idt {
    entries: usize,
}
impl Idt {
    pub fn install(&mut self, n: usize) {
        self.entries = n;
    }
    pub fn entries(&self) -> usize {
        self.entries
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn install_idt() {
        let mut i = Idt::default();
        i.install(256);
        assert_eq!(i.entries(), 256);
    }
}
