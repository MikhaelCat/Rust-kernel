#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrapFrame {
    pub pc: u64,
    pub sp: u64,
}
impl TrapFrame {
    pub fn empty() -> Self {
        Self { pc: 0, sp: 0 }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trap_empty() {
        assert_eq!(TrapFrame::empty().pc, 0);
    }
}
