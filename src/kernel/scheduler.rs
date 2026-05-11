use std::collections::VecDeque;

use super::error::KernelError;
use super::types::{Task, TaskState};

#[derive(Debug, Default)]
pub struct Scheduler {
    next_pid: u32,
    runq: VecDeque<Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            next_pid: 1,
            runq: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, name: &str) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;
        self.runq.push_back(Task {
            pid,
            name: name.to_string(),
            state: TaskState::Ready,
        });
        pid
    }

    pub fn schedule_next(&mut self) -> Result<Task, KernelError> {
        let mut task = self.runq.pop_front().ok_or(KernelError::EmptyRunQueue)?;
        task.state = TaskState::Running;
        let running = task.clone();
        task.state = TaskState::Ready;
        self.runq.push_back(task);
        Ok(running)
    }

    pub fn stop(&mut self, pid: u32) -> Result<(), KernelError> {
        for t in &mut self.runq {
            if t.pid == pid {
                t.state = TaskState::Stopped;
                return Ok(());
            }
        }
        Err(KernelError::TaskNotFound)
    }

    pub fn runnable_count(&self) -> usize {
        self.runq
            .iter()
            .filter(|t| matches!(t.state, TaskState::Ready | TaskState::Running))
            .count()
    }
}
