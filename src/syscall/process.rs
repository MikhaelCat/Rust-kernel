pub fn sys_exit(code: u64) -> i64 {
    -(code as i64)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exit_code() {
        assert_eq!(sys_exit(2), -2);
    }
}
