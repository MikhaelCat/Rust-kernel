use std::collections::BTreeMap;

use super::error::KernelError;
use super::irq::IrqController;
use super::runqueue::RunQueue;
use super::scheduler::Scheduler;
use super::timer::TimerWheel;
use super::types::Process;

pub struct KernelManager {
    scheduler: Scheduler,
    runqueue: RunQueue,
    processes: BTreeMap<u32, Process>,
    timer: TimerWheel,
    irq: IrqController,
}

impl KernelManager {
    pub fn new() -> Self {
        Self {
            scheduler: Scheduler::new(),
            runqueue: RunQueue::default(),
            processes: BTreeMap::new(),
            timer: TimerWheel::default(),
            irq: IrqController::default(),
        }
    }

    pub fn spawn_process(&mut self, name: &str, ppid: u32) -> u32 {
        let pid = self.scheduler.spawn(name);
        self.processes.insert(pid, Process::new(pid, ppid, name));
        self.runqueue.push(pid);
        pid
    }

    pub fn tick(&mut self) -> Result<u32, KernelError> {
        self.timer.advance(1);
        self.irq.handle_irq();
        let task = self.scheduler.schedule_next()?;
        Ok(task.pid)
    }

    pub fn stop_process(&mut self, pid: u32) -> Result<(), KernelError> {
        if !self.processes.contains_key(&pid) {
            return Err(KernelError::ProcessNotFound);
        }
        self.scheduler.stop(pid)?;
        Ok(())
    }

    pub fn jiffies(&self) -> u64 {
        self.timer.now()
    }

    pub fn irq_handled(&self) -> u64 {
        self.irq.handled()
    }
}

impl Default for KernelManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn end_to_end_spawn_and_tick() {
        let mut k = KernelManager::new();
        let p1 = k.spawn_process("init", 0);
        let p2 = k.spawn_process("kthreadd", p1);

        let t1 = k.tick().expect("tick failed");
        let t2 = k.tick().expect("tick failed");

        assert_eq!(t1, p1);
        assert_eq!(t2, p2);
        assert_eq!(k.jiffies(), 2);
        assert_eq!(k.irq_handled(), 2);
    }
}
