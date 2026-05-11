#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoOp {
    Nop,
    Read,
    Write,
}
