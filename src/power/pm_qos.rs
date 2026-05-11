#[derive(Debug, Default)]
pub struct PmQos {
    latency_us: u32,
}
impl PmQos {
    pub fn set_latency(&mut self, v: u32) {
        self.latency_us = v;
    }
    pub fn latency(&self) -> u32 {
        self.latency_us
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_latency() {
        let mut q = PmQos::default();
        q.set_latency(50);
        assert_eq!(q.latency(), 50);
    }
}
