#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockError {
    QueueFull,
    DeviceNotSet,
    InvalidRequest,
    RequestNotFound,
}
