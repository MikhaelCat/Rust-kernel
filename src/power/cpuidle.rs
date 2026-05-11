#[derive(Debug, Default)]
pub struct CpuIdle {
    pub entered: u64,
}
impl CpuIdle {
    pub fn enter(&mut self) {
        self.entered += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enter_idle() {
        let mut c = CpuIdle::default();
        c.enter();
        assert_eq!(c.entered, 1);
    }
}
