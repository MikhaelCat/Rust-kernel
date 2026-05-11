#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeStatus {
    pub jiffies: u64,
    pub now_ns: u64,
}
