use super::op::IoOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sqe {
    pub id: u64,
    pub op: IoOp,
    pub len: u32,
}
