use super::{SyscallError, SyscallTable};
pub fn dispatch(table: &SyscallTable, num: u64, args: &[u64]) -> Result<i64, SyscallError> {
    table.invoke(num, args)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::syscall::sys_getpid;
    #[test]
    fn dispatch_ok() {
        let mut t = SyscallTable::new();
        t.register(39, sys_getpid);
        assert_eq!(dispatch(&t, 39, &[]).expect("ok"), 1);
    }
}
