#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Relocation {
    pub from: u64,
    pub to: u64,
}
impl Relocation {
    pub fn apply(&self, entry: u64) -> u64 {
        entry - self.from + self.to
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relocate_entry() {
        let r = Relocation {
            from: 0x1000,
            to: 0x8000,
        };
        assert_eq!(r.apply(0x1100), 0x8100);
    }
}
