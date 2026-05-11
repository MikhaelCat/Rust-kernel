#[derive(Debug, Default)]
pub struct ResetController {
    asserts: u64,
}
impl ResetController {
    pub fn assert_line(&mut self) {
        self.asserts += 1;
    }
    pub fn asserts(&self) -> u64 {
        self.asserts
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_reset() {
        let mut r = ResetController::default();
        r.assert_line();
        assert_eq!(r.asserts(), 1);
    }
}
