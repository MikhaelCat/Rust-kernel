#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmSnapshot {
    pub vm_id: u64,
    pub generation: u64,
}

impl VmSnapshot {
    pub fn new(vm_id: u64, generation: u64) -> Self {
        Self { vm_id, generation }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_snapshot() {
        let s = VmSnapshot::new(1, 2);
        assert_eq!(s.generation, 2);
    }
}
