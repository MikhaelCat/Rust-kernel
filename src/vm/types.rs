//! VM types for virtual memory management

#[derive(Debug, Clone)]
pub struct Vm {
    pub pid: u32,
    pub start_addr: usize,
    pub end_addr: usize,
}

impl Vm {
    pub fn new(pid: u32, start_addr: usize, end_addr: usize) -> Self {
        Self { pid, start_addr, end_addr }
    }
}
