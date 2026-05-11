#[derive(Debug, Default)]
pub struct Microcode {
    version: u32,
}
impl Microcode {
    pub fn load(&mut self, v: u32) {
        self.version = v;
    }
    pub fn version(&self) -> u32 {
        self.version
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_ucode() {
        let mut m = Microcode::default();
        m.load(7);
        assert_eq!(m.version(), 7);
    }
}
