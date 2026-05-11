use std::collections::BTreeMap;
#[derive(Debug, Default)]
pub struct RegMap {
    regs: BTreeMap<u32, u32>,
}
impl RegMap {
    pub fn write(&mut self, r: u32, v: u32) {
        self.regs.insert(r, v);
    }
    pub fn read(&self, r: u32) -> Option<u32> {
        self.regs.get(&r).copied()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rw_reg() {
        let mut m = RegMap::default();
        m.write(1, 42);
        assert_eq!(m.read(1), Some(42));
    }
}
