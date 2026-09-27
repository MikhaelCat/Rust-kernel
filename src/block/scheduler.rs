//! Advanced I/O Scheduler Algorithms for Linux Kernel on Rust
//! Дополнительные планировщики I/O для оптимизации блочных операций

use super::layer::{BlockRequest, IO_Error, IScheduler};

// ============================================================================
// STUB IMPLEMENTATION FOR SCHEDULER MODULE
// The actual scheduler logic is in layer.rs
// This file serves as a stub and documentation

/// Типы планировщиков I/O
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerType {
    None,           // No scheduling - passthrough mode
    Noop,           // FIFO ordering (best for SSDs)
    Deadline,       // Deadline-based optimization (best for HDDs)
    CFQ,            // Completely Fair Queuing (multi-user fair)
   bfq,             // Budget Fair Queuing (bandwidth guaranteed)
}

impl Default for SchedulerType {
    fn default() -> Self {
        SchedulerType::Deadline  // Best general-purpose choice
    }
}

/// Переход от слоя layer к интерфейсу scheduler
pub use super::layer::{NoopScheduler, DeadlineScheduler, CfqScheduler};

// ============================================================================
// ADVANCED SCHEDULING FEATURES
// ============================================================================

/// ELEVATOR algorithm state tracking
#[derive(Debug)]
pub struct ElevatorState {
    pub last_sector: u64,
    pub direction: Direction,
    pub max_sectors_gap: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    Descending,
    None,
}

impl Default for ElevatorState {
    fn default() -> Self {
        Self {
            last_sector: 0,
            direction: Direction::None,
            max_sectors_gap: 128 * 1024, // 128KB gap threshold
        }
    }
}

impl ElevatorState {
    /// Check if request matches current elevator direction
    pub fn would_accelerate(&self, sector: u64) -> bool {
        match self.direction {
            Direction::Ascending => sector >= self.last_sector,
            Direction::Descending => sector <= self.last_sector,
            Direction::None => true, // First request sets direction
        }
    }
    
    /// Update state after request completion
    pub fn update(&mut self, sector: u64, sectors: usize) {
        self.last_sector = sector + sectors as u64;
        
        if sector > self.last_sector {
            self.direction = Direction::Descending;
        } else {
            self.direction = Direction::Ascending;
        }
    }
}

// ============================================================================
// REQUEST COALESCING
// ============================================================================

/// Attempt to coalesce adjacent requests
pub fn try_coalesce(requests: &mut [BlockRequest]) -> Vec<BlockRequest> {
    if requests.len() < 2 {
        return requests.to_vec();
    }
    
    // Sort by sector
    requests.sort_by_key(|r| r.sector);
    
    let mut merged = Vec::with_capacity(requests.len());
    let mut current = requests[0].clone();
    
    for i in 1..requests.len() {
        let next = &requests[i];
        
        // Can we merge?
        if current.dev_id == next.dev_id &&
           current.operation == next.operation &&
           current.sector + current.sectors as u64 == next.sector
        {
            // Merge
            current.sectors += next.sectors;
            
            // Combine data if present
            if current.data.is_none() && next.data.is_some() {
                current.data = next.data.clone();
            }
        } else {
            // Commit current and start new
            merged.push(current);
            current = next.clone();
        }
    }
    
    // Don't forget the last one
    merged.push(current);
    
    merged
}

// ============================================================================
// I/O PRIORITY CLASSIFICATION
// ============================================================================

/// I/O priority levels (Linux uses 0-7)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IOPriority(pub u8);

impl IOPriority {
    pub const BEST_EFFORT: IOPriority = IOPriority(0);  // Normal processes
    pub const REALTIME: IOPriority = IOPriority(1);     // Real-time processes
    
    pub const MIN_PRIORITY: IOPriority = IOPriority(0);
    pub const MAX_PRIORITY: IOPriority = IOPriority(7);
    
    /// Check if priority is real-time
    pub fn is_realtime(self) -> bool {
        self.0 == Self::REALTIME.0
    }
    
    /// Get numeric priority value (lower = higher priority)
    pub fn value(self) -> u8 {
        self.0
    }
}

/// Priority-aware scheduling helper
pub fn priority_sort(requests: &mut [BlockRequest]) {
    requests.sort_by(|a, b| {
        // For simplicity, assume all are BE level
        // In production, would extract from request metadata
        a.sector.cmp(&b.sector)
    });
}

// ============================================================================
// WRITE BEHIND OPTIMIZATION
// ============================================================================

/// Write behavior configuration
#[derive(Debug, Clone, Copy)]
pub struct WriteBehindConfig {
    pub enabled: bool,
    pub max_bytes: usize,
    pub timeout_ms: u64,
}

impl Default for WriteBehindConfig {
    fn default() -> Self {
        Self {
            enabled: false,         // Disabled by default for safety
            max_bytes: 1024 * 1024, // 1MB max buffer
            timeout_ms: 500,
        }
    }
}

/// Pending write cache for write-behind optimization
pub struct WriteBehindCache {
    config: WriteBehindConfig,
    pending_writes: VecDeque<PendingWrite>,
    total_bytes: usize,
}

struct PendingWrite {
    request: BlockRequest,
    timestamp_ns: u64,
}

impl WriteBehindCache {
    pub fn new(config: WriteBehindConfig) -> Self {
        Self {
            config,
            pending_writes: VecDeque::new(),
            total_bytes: 0,
        }
    }
    
    pub fn can_enqueue(&self) -> bool {
        if !self.config.enabled {
            return false;
        }
        
        self.total_bytes < self.config.max_bytes
    }
    
    pub fn enqueue(&mut self, request: BlockRequest) -> bool {
        if !self.can_enqueue() {
            return false;
        }
        
        let size = request.size();
        self.pending_writes.push_back(PendingWrite {
            request,
            timestamp_ns: get_timestamp_ns(),
        });
        self.total_bytes += size;
        
        true
    }
    
    pub fn flush_expired(&mut self) -> Vec<BlockRequest> {
        if !self.config.enabled {
            return Vec::new();
        }
        
        let now = get_timestamp_ns();
        let timeout_ns = self.config.timeout_ms * 1_000_000;
        
        let mut expired = Vec::new();
        let mut remaining_total = 0;
        
        while let Some(write) = self.pending_writes.front() {
            if now - write.timestamp_ns >= timeout_ns {
                expired.push(self.pending_writes.pop_front().unwrap().request);
            } else {
                // Not expired, keep it and stop processing
                break;
            }
        }
        
        self.total_bytes = self.pending_writes.iter().map(|w| w.request.size()).sum();
        
        expired
    }
}

fn get_timestamp_ns() -> u64 {
    // Simplified - in production use high-resolution timer
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

// ============================================================================
// AVOID READ-WRITE STARVATION
// ============================================================================

/// Prevent read starvation when there's heavy write load
#[derive(Debug)]
pub struct StarvationPrevention {
    pub enable: bool,
    pub max_write_consecutive: usize,
    pub writes_since_last_read: usize,
    pub read_queue_depth: usize,
}

impl Default for StarvationPrevention {
    fn default() -> Self {
        Self {
            enable: true,
            max_write_consecutive: 32,
            writes_since_last_read: 0,
            read_queue_depth: 0,
        }
    }
}

impl StarvationPrevention {
    pub fn record_write(&mut self) {
        self.writes_since_last_read += 1;
    }
    
    pub fn record_read(&mut self) {
        self.writes_since_last_read = 0;
    }
    
    pub fn should_force_read(&self) -> bool {
        if !self.enable {
            return false;
        }
        
        self.writes_since_last_read >= self.max_write_consecutive ||
        self.read_queue_depth > self.max_write_consecutive * 2
    }
}

// ============================================================================
// DISCARD/TRIM HANDLING
// ============================================================================

/// Handle discards efficiently
#[derive(Debug)]
pub struct DiscardHandler {
    pub enabled: bool,
    pub max_discard_sectors: usize,
    pub discard_alignment: usize,
}

impl Default for DiscardHandler {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled unless device supports TRIM
            max_discard_sectors: 128 * 1024, // 128K sectors max
            discard_alignment: 4096,
        }
    }
}

// ============================================================================
// STATISTICS ACCUMULATION
// ============================================================================

#[derive(Debug)]
pub struct SchedulerStats {
    pub total_requests: AtomicU64,
    pub merged_requests: AtomicU64,
    pub dispatched_requests: AtomicU64,
    pub completed_requests: AtomicU64,
    pub dropped_requests: AtomicU64,
    
    pub avg_latency_ns: AtomicU64,
    pub max_latency_ns: AtomicU64,
    pub throughput_kb_per_sec: AtomicU64,
    
    pub read_requests: AtomicU64,
    pub write_requests: AtomicU64,
}

impl SchedulerStats {
    pub fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            merged_requests: AtomicU64::new(0),
            dispatched_requests: AtomicU64::new(0),
            completed_requests: AtomicU64::new(0),
            dropped_requests: AtomicU64::new(0),
            avg_latency_ns: AtomicU64::new(0),
            max_latency_ns: AtomicU64::new(0),
            throughput_kb_per_sec: AtomicU64::new(0),
            read_requests: AtomicU64::new(0),
            write_requests: AtomicU64::new(0),
        }
    }
    
    pub fn record_request(&self, is_write: bool) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        
        if is_write {
            self.write_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            self.read_requests.fetch_add(1, Ordering::Relaxed);
        }
    }
}

// ============================================================================
// TESTING UTILITIES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_elevator_direction_tracking() {
        let mut state = ElevatorState::default();
        
        assert!(state.would_accelerate(100));
        state.update(100, 8);
        
        // Now expect ascending
        assert!(!state.would_accelerate(50));
        assert!(state.would_accelerate(200));
    }
    
    #[test]
    fn test_request_coalescing() {
        let mut requests = vec![
            BlockRequest::read(0, 100, 8),
            BlockRequest::read(0, 108, 8),  // Adjacent to first
            BlockRequest::read(0, 200, 8),  // Gap
        ];
        
        let merged = try_coalesce(&mut requests);
        
        // Should have 2 requests: merged(100,16) and separate(200,8)
        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].sector, 100);
        assert_eq!(merged[0].sectors, 16);
        assert_eq!(merged[1].sector, 200);
        assert_eq!(merged[1].sectors, 8);
    }
    
    #[test]
    fn test_io_priority() {
        assert!(IOPriority::BEST_EFFORT.is_realtime());
        assert!(!IOPriority::MAX_PRIORITY.is_realtime());
        assert_eq!(IOPriority::REALTIME.value(), 1);
    }
    
    #[test]
    fn test_starvation_prevention() {
        let mut prevention = StarvationPrevention::default();
        
        // Initially no starvation risk
        assert!(!prevention.should_force_read());
        
        // Simulate write flood
        for _ in 0..40 {
            prevention.record_write();
        }
        
        // Should force read after threshold
        assert!(prevention.should_force_read());
    }
    
    #[test]
    fn test_scheduler_stats() {
        let stats = SchedulerStats::new();
        
        stats.record_request(true);  // Write
        stats.record_request(false); // Read
        
        assert_eq!(stats.total_requests.load(Ordering::Relaxed), 2);
        assert_eq!(stats.write_requests.load(Ordering::Relaxed), 1);
        assert_eq!(stats.read_requests.load(Ordering::Relaxed), 1);
    }
}
