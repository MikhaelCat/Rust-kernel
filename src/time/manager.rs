//! Timer Management System

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Timer {
    pub id: u32,
    pub expires_at: u64,
    pub handler_id: u32,
    pub repeats: bool,
}

#[derive(Debug, Clone)]
pub struct TimerHandler {
    pub handler_id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct TimerSubsystem {
    pub timers: BTreeMap<u64, Timer>,
    pub handlers: Vec<TimerHandler>,
    pub next_id: u32,
    pub current_time: u64,
}

impl Default for TimerSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

impl TimerSubsystem {
    pub fn new() -> Self {
        Self {
            timers: BTreeMap::new(),
            handlers: Vec::new(),
            next_id: 1,
            current_time: 0,
        }
    }

    pub fn add_handler(&mut self, name: &str) -> u32 {
        let id = self.next_id;
        self.handlers.push(TimerHandler {
            handler_id: id,
            name: name.to_string(),
        });
        self.next_id += 1;
        id
    }

    pub fn start_timer(&mut self, timeout_ns: u64, handler_id: u32) -> u32 {
        let id = self.next_id;
        let expires_at = self.current_time + timeout_ns;
        
        self.timers.insert(expires_at, Timer {
            id,
            expires_at,
            handler_id,
            repeats: false,
        });
        
        self.next_id += 1;
        id
    }

    pub fn stop_timer(&mut self, timer_id: u32) -> bool {
        self.timers.retain(|_, t| t.id != timer_id);
        true
    }

    pub fn advance_time(&mut self, delta_ns: u64) -> Vec<u32> {
        self.current_time += delta_ns;
        
        let mut expired_handlers = Vec::new();
        let current_time = self.current_time;
        
        // Find and remove expired timers
        let to_remove: Vec<u64> = self.timers.keys()
            .filter(|&&t| t <= current_time)
            .copied()
            .collect();
        
        for expires_at in to_remove {
            if let Some(timer) = self.timers.remove(&expires_at) {
                expired_handlers.push(timer.handler_id);
            }
        }
        
        expired_handlers
    }

    pub fn pending_timers(&self) -> usize {
        self.timers.len()
    }

    pub fn next_expiration(&self) -> Option<u64> {
        self.timers.keys().next().copied()
    }
}
