#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub pid: u32,
    pub name: String,
    pub state: TaskState,
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
