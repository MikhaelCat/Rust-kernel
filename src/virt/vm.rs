#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vm {
    pub id: u64,
    pub running: bool,
}

impl Vm {
    pub fn new(id: u64) -> Self {
        Self { id, running: false }
    }

    pub fn start(&mut self) {
        self.running = true;
    }
}
