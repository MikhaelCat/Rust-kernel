#[derive(Debug, Default)]
pub struct Rps {
    flows: u64,
}
impl Rps {
    pub fn steer(&mut self) {
        self.flows += 1;
    }
    pub fn flows(&self) -> u64 {
        self.flows
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn steer_flow() {
        let mut r = Rps::default();
        r.steer();
        assert_eq!(r.flows(), 1);
    }
}
