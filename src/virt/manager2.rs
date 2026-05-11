use super::manager::VirtManager;

#[derive(Debug, Default)]
pub struct VirtFleet {
    mgr: VirtManager,
}

impl VirtFleet {
    pub fn spawn(&mut self) -> u64 {
        let id = self.mgr.create_vm(512, [0, 0, 0, 0, 0, 1], 500_000);
        let _ = self.mgr.boot_vm(id);
        id
    }

    pub fn count_running(&self) -> usize {
        self.mgr.running_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_running_vm() {
        let mut f = VirtFleet::default();
        let _ = f.spawn();
        assert_eq!(f.count_running(), 1);
    }
}
