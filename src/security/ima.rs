#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImaResult {
    Pass,
    Fail,
}
pub fn measure(hash_ok: bool) -> ImaResult {
    if hash_ok {
        ImaResult::Pass
    } else {
        ImaResult::Fail
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ima_fail() {
        assert_eq!(measure(false), ImaResult::Fail);
    }
}
