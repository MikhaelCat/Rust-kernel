#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedError {
    Empty,
    NoSuchProcess,
    InvalidState,
}
