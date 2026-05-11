#[derive(Debug, Default)]
pub struct FairClass {
    pub slices: u64,
}
impl FairClass {
    pub fn grant(&mut self) {
        self.slices += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grant_slice() {
        let mut c = FairClass::default();
        c.grant();
        assert_eq!(c.slices, 1);
    }
}
