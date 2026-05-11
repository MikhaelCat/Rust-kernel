#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OomAction {
    KillBiggest,
    Panic,
}
pub fn decide(free_pages: usize) -> OomAction {
    if free_pages == 0 {
        OomAction::KillBiggest
    } else {
        OomAction::Panic
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oom_decision() {
        assert_eq!(decide(0), OomAction::KillBiggest);
    }
}
