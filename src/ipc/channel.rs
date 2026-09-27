//! Inter-Process Communication Implementation

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u32,
    pub data: Vec<u8>,
    pub sender_pid: u32,
    pub timestamp: u64,
}

impl Message {
    pub fn new(id: u32, data: &[u8], sender: u32) -> Self {
        Self {
            id,
            data: data.to_vec(),
            sender_pid: sender,
            timestamp: 0,
        }
    }
}

#[derive(Debug)]
pub struct MessageQueue {
    pub messages: Vec<Message>,
    pub queue_id: u32,
}

impl Default for MessageQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageQueue {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            queue_id: 0,
        }
    }

    pub fn send(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn receive(&mut self) -> Option<Message> {
        if self.messages.is_empty() {
            None
        } else {
            Some(self.messages.remove(0))
        }
    }

    pub fn count(&self) -> usize {
        self.messages.len()
    }
}

#[derive(Debug)]
pub struct Semaphore {
    pub semaphore_id: u32,
    pub value: i16,
    pub processes: HashMap<u32, bool>,
}

impl Default for Semaphore {
    fn default() -> Self {
        Self::new()
    }
}

impl Semaphore {
    pub fn new(sem_id: u32) -> Self {
        Self {
            semaphore_id: sem_id,
            value: 1,
            processes: HashMap::new(),
        }
    }

    pub fn acquire(&mut self) -> Result<(), &'static str> {
        if self.value > 0 {
            self.value -= 1;
            Ok(())
        } else {
            Err("Semaphore unavailable")
        }
    }

    pub fn release(&mut self) {
        self.value += 1;
    }

    pub fn get_value(&self) -> i16 {
        self.value
    }
}
