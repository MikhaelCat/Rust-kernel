#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitResult {
    pub pid: i64,
    pub status: i64,
}

pub fn make_wait_status(exit_code: i64) -> i64 {
    exit_code & 0xff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_low_byte() {
        assert_eq!(make_wait_status(260), 4);
    }
}
