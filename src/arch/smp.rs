#[derive(Debug, Default)]
pub struct SmpState {
    cpus_online: u32,
}
impl SmpState {
    pub fn cpu_up(&mut self) {
        self.cpus_online += 1;
    }
    pub fn online(&self) -> u32 {
        self.cpus_online
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cpu_up() {
        let mut s = SmpState::default();
        s.cpu_up();
        assert_eq!(s.online(), 1);
    }
}
