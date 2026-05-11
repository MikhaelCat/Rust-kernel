#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallAbi {
    pub num: u64,
    pub args: [u64; 6],
}
impl SyscallAbi {
    pub fn new(num: u64) -> Self {
        Self { num, args: [0; 6] }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn abi_new() {
        assert_eq!(SyscallAbi::new(60).num, 60);
    }
}
