use super::suspend::SuspendState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerStatus {
    pub frequency_khz: u64,
    pub suspend_state: SuspendState,
}
