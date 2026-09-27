//! Types and Data Structures for Linux Kernel on Rust

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Stopped,
    Zombie,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub pid: u32,
    pub name: String,
    pub state: TaskState,
    pub prio: u8,
    pub nice: i8,
}

impl Task {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            name: name.to_string(),
            state: TaskState::Ready,
            prio: 20,
            nice: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Process {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
}

impl Process {
    pub fn new(pid: u32, ppid: u32, name: &str) -> Self {
        Self {
            pid,
            ppid,
            name: name.to_string(),
        }
    }
}
