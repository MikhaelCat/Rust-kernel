use super::lifecycle::VmState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmCheckpoint {
    pub vm_id: u64,
    pub state: VmState,
    pub mem_effective_mb: u32,
    pub net_up: bool,
}
