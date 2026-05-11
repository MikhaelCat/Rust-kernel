use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyscallError {
    UnknownNumber,
}

pub type SysResult = Result<i64, SyscallError>;
pub type SysHandler = fn(&[u64]) -> SysResult;

#[derive(Default)]
pub struct SyscallTable {
    handlers: BTreeMap<u64, SysHandler>,
}

impl SyscallTable {
    pub fn new() -> Self {
        Self {
            handlers: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, num: u64, handler: SysHandler) {
        self.handlers.insert(num, handler);
    }

    pub fn invoke(&self, num: u64, args: &[u64]) -> SysResult {
        let handler = self.handlers.get(&num).ok_or(SyscallError::UnknownNumber)?;
        handler(args)
    }
}

pub fn sys_getpid(args: &[u64]) -> SysResult {
    if args.is_empty() {
        return Ok(1);
    }
    Ok(args[0] as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoke_registered_syscall() {
        let mut t = SyscallTable::new();
        t.register(39, sys_getpid);
        assert_eq!(t.invoke(39, &[]).expect("syscall failed"), 1);
    }

    #[test]
    fn unknown_syscall_is_error() {
        let t = SyscallTable::new();
        assert_eq!(
            t.invoke(999, &[]).expect_err("must fail"),
            SyscallError::UnknownNumber
        );
    }
}
