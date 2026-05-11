#[derive(Debug, Default)]
pub struct Preempt {
    count: u32,
}
impl Preempt {
    pub fn disable(&mut self) {
        self.count += 1;
    }
    pub fn enable(&mut self) {
        if self.count > 0 {
            self.count -= 1;
        }
    }
    pub fn count(&self) -> u32 {
        self.count
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preempt_toggle() {
        let mut p = Preempt::default();
        p.disable();
        p.enable();
        assert_eq!(p.count(), 0);
    }
}
