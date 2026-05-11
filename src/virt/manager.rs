use super::block::VirtBlock;
use super::checkpoint::VmCheckpoint;
use super::device::{DeviceSet, VirtDevice};
use super::lifecycle::{VmLifecycle, VmState};
use super::memory::VmMemory;
use super::migration::{MigrationPlan, MigrationResult, validate_plan};
use super::network::VirtNet;

#[derive(Debug)]
pub struct Vm {
    pub id: u64,
    pub life: VmLifecycle,
    pub mem: VmMemory,
    pub net: VirtNet,
    pub blk: VirtBlock,
    pub devices: DeviceSet,
}

impl Vm {
    pub fn new(id: u64, mem_mb: u32, mac: [u8; 6], sectors: u64) -> Self {
        let mut devices = DeviceSet::default();
        devices.add(VirtDevice::Console);
        devices.add(VirtDevice::Net {
            name: format!("virtio-net{id}"),
        });
        devices.add(VirtDevice::Block {
            name: format!("virtio-blk{id}"),
        });

        Self {
            id,
            life: VmLifecycle::new(id),
            mem: VmMemory::new(mem_mb),
            net: VirtNet::new(mac),
            blk: VirtBlock::new(sectors),
            devices,
        }
    }

    pub fn boot(&mut self) -> bool {
        if !self.life.start() {
            return false;
        }
        self.net.up();
        true
    }

    pub fn pause(&mut self) -> bool {
        if !self.life.pause() {
            return false;
        }
        self.net.link_up = false;
        true
    }

    pub fn resume(&mut self) -> bool {
        if !self.life.resume() {
            return false;
        }
        self.net.up();
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.life.stop() {
            return false;
        }
        self.net.link_up = false;
        true
    }

    pub fn reset(&mut self) -> bool {
        if !self.life.reset() {
            return false;
        }
        self.mem.balloon_mb = 0;
        self.net.link_up = false;
        true
    }

    pub fn healthy(&self) -> bool {
        self.life.state == VmState::Running && self.net.link_up && self.devices.has_net()
    }

    pub fn checkpoint(&self) -> VmCheckpoint {
        VmCheckpoint {
            vm_id: self.id,
            state: self.life.state,
            mem_effective_mb: self.mem.effective_mb(),
            net_up: self.net.link_up,
        }
    }

    pub fn restore_from_checkpoint(&mut self, cp: &VmCheckpoint) -> bool {
        if cp.vm_id != self.id {
            return false;
        }
        self.life.state = cp.state;
        self.net.link_up = cp.net_up;
        true
    }
}

#[derive(Debug, Default)]
pub struct VirtManager {
    vms: Vec<Vm>,
    next_id: u64,
}

impl VirtManager {
    pub fn create_vm(&mut self, mem_mb: u32, mac: [u8; 6], sectors: u64) -> u64 {
        self.next_id += 1;
        let id = self.next_id;
        self.vms.push(Vm::new(id, mem_mb, mac, sectors));
        id
    }

    pub fn boot_vm(&mut self, id: u64) -> bool {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.id == id) {
            return vm.boot();
        }
        false
    }

    pub fn pause_vm(&mut self, id: u64) -> bool {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.id == id) {
            return vm.pause();
        }
        false
    }

    pub fn resume_vm(&mut self, id: u64) -> bool {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.id == id) {
            return vm.resume();
        }
        false
    }

    pub fn stop_vm(&mut self, id: u64) -> bool {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.id == id) {
            return vm.stop();
        }
        false
    }

    pub fn reset_vm(&mut self, id: u64) -> bool {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.id == id) {
            return vm.reset();
        }
        false
    }

    pub fn balloon_vm(&mut self, id: u64, mb: u32) -> bool {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.id == id) {
            vm.mem.balloon(mb);
            return true;
        }
        false
    }

    pub fn vm_healthy(&self, id: u64) -> bool {
        self.vms
            .iter()
            .find(|v| v.id == id)
            .map(|v| v.healthy())
            .unwrap_or(false)
    }

    pub fn running_count(&self) -> usize {
        self.vms
            .iter()
            .filter(|v| v.life.state == VmState::Running)
            .count()
    }

    pub fn checkpoint_vm(&self, id: u64) -> Option<VmCheckpoint> {
        self.vms.iter().find(|v| v.id == id).map(|v| v.checkpoint())
    }

    pub fn restore_vm(&mut self, cp: &VmCheckpoint) -> bool {
        if let Some(vm) = self.vms.iter_mut().find(|v| v.id == cp.vm_id) {
            return vm.restore_from_checkpoint(cp);
        }
        false
    }

    pub fn migrate_vm(&self, id: u64, source: &str, target: &str) -> MigrationResult {
        if self.vms.iter().any(|v| v.id == id) {
            let plan = MigrationPlan {
                vm_id: id,
                source: source.to_string(),
                target: target.to_string(),
            };
            return validate_plan(&plan);
        }
        MigrationResult::Reject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_full_flow() {
        let mut m = VirtManager::default();
        let id = m.create_vm(1024, [0, 1, 2, 3, 4, 5], 1_000_000);
        assert!(m.boot_vm(id));
        assert!(m.balloon_vm(id, 128));
        assert!(m.vm_healthy(id));
        assert_eq!(m.running_count(), 1);
    }

    #[test]
    fn checkpoint_restore_and_migrate() {
        let mut m = VirtManager::default();
        let id = m.create_vm(512, [0, 0, 0, 0, 0, 1], 100_000);
        assert!(m.boot_vm(id));

        let cp = m.checkpoint_vm(id).expect("checkpoint missing");
        assert!(m.restore_vm(&cp));

        assert_eq!(
            m.migrate_vm(id, "node-a", "node-b"),
            MigrationResult::Success
        );
    }

    #[test]
    fn lifecycle_control_flow() {
        let mut m = VirtManager::default();
        let id = m.create_vm(512, [0, 1, 2, 3, 4, 5], 100_000);
        assert!(m.boot_vm(id));
        assert!(m.pause_vm(id));
        assert!(!m.vm_healthy(id));
        assert!(m.resume_vm(id));
        assert!(m.vm_healthy(id));
        assert!(m.stop_vm(id));
        assert_eq!(m.running_count(), 0);
        assert!(m.reset_vm(id));
        assert!(m.boot_vm(id));
        assert!(m.vm_healthy(id));
    }

    #[test]
    fn invalid_lifecycle_actions_are_rejected() {
        let mut m = VirtManager::default();
        let id = m.create_vm(128, [1, 1, 1, 1, 1, 1], 1_000);
        assert!(!m.pause_vm(id));
        assert!(!m.resume_vm(id));
        assert!(!m.stop_vm(id));
        assert!(m.boot_vm(id));
        assert!(!m.boot_vm(id));
        assert!(!m.reset_vm(id));
    }
}
