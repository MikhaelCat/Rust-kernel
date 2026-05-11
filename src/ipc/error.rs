#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcCoreError {
    QueueEmpty,
    NoSuchChannel,
}
