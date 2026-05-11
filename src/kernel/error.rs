#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelError {
    EmptyRunQueue,
    TaskNotFound,
    ProcessNotFound,
}
