//! Сcheduler for Linux Kernel - All Algorithms

use super::types::{Task, TaskState};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Scheduler {
    next_pid: u32,
    runq: VecDeque<Task>,
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
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
        let task = Task::new(pid, name);
        self.runq.push_back(task);
        pid
    }

    pub fn schedule_next(&mut self) -> Option<Task> {
        let mut task = self.runq.pop_front()?;
        task.state = TaskState::Running;
        let running = task.clone();
        task.state = TaskState::Ready;
        self.runq.push_back(task);
        Some(running)
    }

    pub fn stop(&mut self, pid: u32) -> bool {
        if let Some(pos) = self.runq.iter().position(|t| t.pid == pid) {
            self.runq[pos].state = TaskState::Stopped;
            true
        } else {
            false
        }
    }

    pub fn runnable_count(&self) -> usize {
        self.runq.len()
    }
}
