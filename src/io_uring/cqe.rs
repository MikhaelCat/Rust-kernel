#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cqe {
    pub id: u64,
    pub res: i32,
}
