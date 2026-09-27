//! Block I/O Queue Implementation

#[derive(Debug, Clone)]
pub struct Request {
    pub sector: u64,
    pub size: usize,
    pub data: Option<Vec<u8>>,
    pub done: bool,
}

impl Request {
    pub fn new(sector: u64, size: usize) -> Self {
        Self {
            sector,
            size,
            data: None,
            done: false,
        }
    }
}

#[derive(Debug)]
pub struct RequestQueue {
    pub requests: Vec<Request>,
    pub queue_depth: u32,
}

impl Default for RequestQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestQueue {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
            queue_depth: 0,
        }
    }

    pub fn add_request(&mut self, request: Request) {
        self.requests.push(request);
        self.queue_depth += 1;
    }

    pub fn process_all(&mut self) -> usize {
        let count = self.requests.len();
        self.requests.iter_mut().for_each(|r| r.done = true);
        count
    }

    pub fn pending_count(&self) -> usize {
        self.requests.iter().filter(|r| !r.done).count()
    }
}
