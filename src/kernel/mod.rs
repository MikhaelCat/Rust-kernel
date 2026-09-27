pub mod component_01;
pub mod component_02;
pub mod component_03;
pub mod component_04;

pub mod error;
pub mod irq;
pub mod manager;
pub mod process;
pub mod runqueue;
pub mod scheduler;
pub mod task;
pub mod timer;
pub mod types;

pub use error::KernelError;
pub use manager::KernelManager;
pub use scheduler::Scheduler;
pub use types::{Process, Task, TaskState};

pub struct KernelCore {
    manager: KernelManager,
}

impl KernelCore {
    pub fn new() -> Self {
        Self {
            manager: KernelManager::new(),
        }
    }

    pub fn spawn_kernel_task(&mut self, name: &str) -> u32 {
        self.manager.spawn_process(name, 0)
    }

    pub fn tick(&mut self) -> Option<u32> {
        self.manager.tick().ok()
    }
}

impl Default for KernelCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_core_ticks_tasks() {
        let mut k = KernelCore::new();
        k.spawn_kernel_task("init");
        assert_eq!(k.tick(), Some(1));
    }

    #[test]
    fn manager_stop_unknown_process() {
        let mut k = KernelManager::new();
        assert_eq!(k.stop_process(999), Err(KernelError::ProcessNotFound));
    }
}
pub mod kallsyms;
pub mod kthread;
pub mod livepatch;
pub mod lockdep;
pub mod notifier;
pub mod paniclog;
pub mod rcu;
pub mod sysctl;
pub mod workqueue;

pub mod status;
pub mod system;
pub use system::KernelSystem;
pub mod health;

pub mod system2;

pub mod components;

pub mod stats;
