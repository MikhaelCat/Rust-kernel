#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryMode {
    None,
    Safe,
}
pub fn choose_recovery(boot_failures: u32) -> RecoveryMode {
    if boot_failures > 3 {
        RecoveryMode::Safe
    } else {
        RecoveryMode::None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn safe_after_retries() {
        assert_eq!(choose_recovery(4), RecoveryMode::Safe);
    }
}
