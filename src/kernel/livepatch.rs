#[derive(Debug, Default)]
pub struct Livepatch {
    applied: bool,
}
impl Livepatch {
    pub fn apply(&mut self) {
        self.applied = true;
    }
    pub fn applied(&self) -> bool {
        self.applied
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn apply_patch() {
        let mut l = Livepatch::default();
        l.apply();
        assert!(l.applied());
    }
}
