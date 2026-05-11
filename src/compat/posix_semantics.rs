#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitStatus {
    Exited(i64),
    Signaled(i32),
}

pub fn normalize_wait_status(s: WaitStatus) -> i64 {
    match s {
        WaitStatus::Exited(code) => code & 0xff,
        WaitStatus::Signaled(sig) => 128 + sig as i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn normalize_status() {
        assert_eq!(normalize_wait_status(WaitStatus::Exited(0)), 0);
        assert_eq!(normalize_wait_status(WaitStatus::Signaled(9)), 137);
    }
}
