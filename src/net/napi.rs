#[derive(Debug, Default)]
pub struct Napi {
    polls: u64,
}
impl Napi {
    pub fn poll(&mut self) {
        self.polls += 1;
    }
    pub fn polls(&self) -> u64 {
        self.polls
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn napi_poll() {
        let mut n = Napi::default();
        n.poll();
        assert_eq!(n.polls(), 1);
    }
}
