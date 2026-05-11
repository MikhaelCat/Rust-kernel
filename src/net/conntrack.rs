#[derive(Debug, Default)]
pub struct Conntrack {
    entries: u64,
}
impl Conntrack {
    pub fn new_flow(&mut self) {
        self.entries += 1;
    }
    pub fn entries(&self) -> u64 {
        self.entries
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_flow() {
        let mut c = Conntrack::default();
        c.new_flow();
        assert_eq!(c.entries(), 1);
    }
}
