//! Scheduling Type Definitions
//! 
//! Типы для планировщика задач

use crate::kernel::types::{Task as KernelTask, TaskState};

/// Расширенная структура задачи с поддержкой RT параметров
#[derive(Debug, Clone)]
pub struct Task {
    pub pid: u32,
    pub comm: String,
    pub state: TaskState,
    pub prio: u8,          // Static priority (1-99 for RT)
    pub nice: i8,          // CFS nice value (-20 to +19)
    pub rt_prio: super::rt::RtPrio,  // Real-time priority
}

impl Default for Task {
    fn default() -> Self {
        Self {
            pid: 0,
            comm: "unknown".to_string(),
            state: TaskState::Ready,
            prio: 20,
            nice: 0,
            rt_prio: super::rt::RtPrio::new(super::rt::RtPrio::NORMAL).unwrap(),
        }
    }
}

impl Task {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            comm: name.to_string(),
            state: TaskState::Ready,
            prio: 20,
            nice: 0,
            rt_prio: super::rt::RtPrio::new(super::rt::RtPrio::NORMAL).unwrap(),
        }
    }
    
    /// Создать реальную задачу
    pub fn new_rt(pid: u32, name: &str, priority: u8) -> Result<Self, super::error::SchedError> {
        let rt_prio = super::rt::RtPrio::new(priority)?;
        Ok(Self {
            pid,
            comm: name.to_string(),
            state: TaskState::Ready,
            prio: priority,
            nice: 0,
            rt_prio,
        })
    }
    
    /// Создать нормальную задачу
    pub fn new_normal(pid: u32, name: &str, nice: i8) -> Self {
        Self {
            pid,
            comm: name.to_string(),
            state: TaskState::Ready,
            prio: 20,
            nice,
            rt_prio: super::rt::RtPrio::new(super::rt::RtPrio::NORMAL).unwrap(),
        }
    }
}

/// Конвертация между Task и KernelTask
impl From<Task> for KernelTask {
    fn from(task: Task) -> Self {
        KernelTask {
            pid: task.pid,
            name: task.comm,
            state: task.state,
            prio: task.prio,
            nice: task.nice,
        }
    }
}

impl From<&Task> for KernelTask {
    fn from(task: &Task) -> Self {
        KernelTask {
            pid: task.pid,
            name: task.comm.clone(),
            state: task.state,
            prio: task.prio,
            nice: task.nice,
        }
    }
}
