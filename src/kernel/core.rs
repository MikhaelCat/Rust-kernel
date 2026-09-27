//! Kernel Core Implementation - Main System Manager

use crate::sched::SchedulerManager;
use crate::mm::PhysicalMemoryManager;
use crate::fs::Vfs;
use crate::net::NetworkStack;
use crate::boot::BootManager;

#[derive(Debug)]
pub struct KernelSystem {
    pub memory_manager: PhysicalMemoryManager,
    pub vfs: Vfs,
    pub scheduler: SchedulerManager,
    pub network: NetworkStack,
    pub boot_manager: BootManager,
}

impl Default for KernelSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl KernelSystem {
    pub fn new() -> Self {
        Self {
            memory_manager: PhysicalMemoryManager::new(4096 * 1024, 4096).unwrap(),
            vfs: Vfs::new(),
            scheduler: SchedulerManager::new(),
            network: NetworkStack::new(),
            boot_manager: BootManager::new(),
        }
    }

    pub fn bootstrap(&mut self) -> Result<(), &'static str> {
        // Initialize filesystem root
        self.vfs.mkdir("/").map_err(|_| "Root creation failed")?;
        self.vfs.mkdir("/tmp").map_err(|_| "Temp dir failed")?;
        self.vfs.mkdir("/proc").map_err(|_| "Proc dir failed")?;
        
        // Create some initial files
        self.vfs.create_file("/etc/passwd", b"root:x:0:0:root:/root:/bin/bash").unwrap();
        
        // Start network stack
        self.network.create_socket(80);
        self.network.create_socket(443);
        self.network.create_socket(22);
        
        Ok(())
    }

    pub fn tick(&mut self) -> Option<u32> {
        // Get next task from scheduler
        self.scheduler.select_next_task()
    }

    pub fn spawn_task(&mut self, name: &str) -> u32 {
        // Simplified task spawning
        let pid = self.boot_manager.kernel_modules.len() as u32 + 1;
        
        // Add to scheduler
        use super::types::Task;
        let mut task = Task::new(pid, name);
        task.prio = 20;
        
        self.scheduler.enqueue_task(&mut task);
        pid
    }

    pub fn total_tasks(&self) -> usize {
        self.scheduler.cpu_count()
    }
}
