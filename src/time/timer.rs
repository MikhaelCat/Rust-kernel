//! Time/Timer System Implementation for Linux Kernel on Rust
//! Реализация системы таймеров и clocksource для ядра Linux на Rust

use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU32, Ordering};
use std::collections::VecDeque;

// ============================================================================
// CLOCK SOURCE INFRASTRUCTURE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockSourceType {
    TSC,            // Time Stamp Counter (fastest)
    HPET,           // High Precision Event Timer
    ACPI_PM,        // ACPI Power Management Timer
    PIT,            // Programmable Interval Timer
}

#[derive(Debug)]
pub struct Clocksource {
    pub name: &'static str,
    pub source_type: ClockSourceType,
    pub mult: u32,                // Multiplier (fixed-point arithmetic)
    pub shift: u32,               // Shift amount
    pub mask: u64,                // Counter mask
    pub frequency: u64,           // Hz (if fixed)
    pub running: AtomicBool,
    pub errors: AtomicU64,
}

impl Clocksource {
    pub fn new(name: &'static str, source_type: ClockSourceType) -> Self {
        Self {
            name,
            source_type,
            mult: 0,
            shift: 0,
            mask: !0u64,
            frequency: 0,
            running: AtomicBool::new(false),
            errors: AtomicU64::new(0),
        }
    }
    
    /// Read raw counter value
    pub fn read(&self) -> u64 {
        unsafe {
            match self.source_type {
                ClockSourceType::TSC => std::arch::x86_64::_rdtsc() as u64,
                _ => {
                    self.errors.fetch_add(1, Ordering::Relaxed);
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_nanos() as u64
                },
            }
        }
    }
    
    /// Convert counter to nanoseconds
    pub fn to_ns(&self, counter: u64) -> u64 {
        let adjusted = if self.shift > 0 {
            ((counter as u128 * self.mult as u128) >> self.shift) as u64
        } else {
            counter
        };
        
        if self.frequency > 0 {
            (adjusted * 1_000_000_000) / self.frequency
        } else {
            adjusted
        }
    }
    
    /// Get current monotonic time in nanoseconds
    pub fn ktime_get_mono_fast_ns(&self) -> u64 {
        if self.running.load(Ordering::Relaxed) {
            self.to_ns(self.read())
        } else {
            0
        }
    }
    
    /// Calibrate against known-good clock
    pub fn calibrate(&mut self, reference_ns: u64) -> u64 {
        let start = self.read();
        let end = start + 1_000_000; // 1 million ticks
        
        self.mwait(end);
        
        let delta = self.read() - start;
        if delta > 0 {
            // Calculate frequency and calibration factor
            self.frequency = (reference_ns * 1_000_000) / delta;
            
            // Set mult/shift for fast conversion
            self.calculate_mult_shift();
        }
        
        delta
    }
    
    fn calculate_mult_shift(&mut self) {
        // Fixed-point math optimization
        self.shift = 32;
        self.mult = ((1u128 << 32) / (self.frequency as u128)).min(u32::MAX as u128) as u32;
    }
    
    fn mwait(&self, target: u64) {
        while self.read() < target {
            unsafe { std::arch::x86_64::_mm_pause(); }
        }
    }
}

/// Default TSC clocksource
fn tsc_clocksource() -> Clocksource {
    let mut cs = Clocksource::new("TSC", ClockSourceType::TSC);
    cs.runing.store(true, Ordering::Relaxed);
    cs.calibrate(1_000_000); // Calibrate with 1ms reference
    cs
}

// ============================================================================
// CLOCK EVENT GENERATORS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClockeventMode {
    Oneshot,              // Single-shot mode
    Periodic,             // Periodic interrupts
    Shutdown,             // Device shutdown
    Resume,               // Wake from low power
    Broadcast,            // Broadcast to all CPUs
}

#[derive(Debug)]
pub struct ClockeventFeatures {
    pub periodic: bool,
    pub oneshot: bool,
    pub low_power: bool,
    pub standby: bool,
    pub broadcast: bool,
}

#[derive(Debug)]
pub struct Clockevent {
    pub name: &'static str,
    pub features: ClockeventFeatures,
    pub rating: u32,              // Quality score (0-500)
    pub irq: Option<u32>,
    pub mode: ClockeventMode,
    pub next_event_ns: u64,
    pub cpu_id: usize,
}

impl Clockevent {
    pub fn new(name: &'static str, cpu_id: usize) -> Self {
        Self {
            name,
            features: ClockeventFeatures {
                periodic: true,
                oneshot: true,
                low_power: true,
                standby: false,
                broadcast: false,
            },
            rating: 300,
            irq: None,
            mode: ClockeventMode::Oneshot,
            next_event_ns: 0,
            cpu_id,
        }
    }
    
    pub fn set_mode(&mut self, mode: ClockeventMode) {
        self.mode = mode;
    }
    
    pub fn set_next_event(&mut self, ns: u64) -> Result<(), TimersError> {
        if ns == 0 {
            return Err(TimersError::InvalidArgument);
        }
        
        self.next_event_ns = ns;
        Ok(())
    }
    
    pub fn arm_for_sleep(&mut self, sleep_ns: u64) -> Result<(), TimersError> {
        if sleep_ns > 1_000_000_000 { // Max 1 second
            return Err(TimersError::InvalidArgument);
        }
        
        self.set_next_event(sleep_ns)
    }
}

// ============================================================================
// HIGH-RESOLUTION TIMERS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerState {
    Inactive,
    Running,
    Expired,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerResult {
    Expire,         // Single-shot timer expired
    RemainRunning,  // Recurring timer continues
}

pub type TimerCallback = Box<dyn Fn() -> TimerResult + Send>;

#[derive(Debug)]
pub struct Hrtimer {
    pub id: u64,
    pub callback: TimerCallback,
    pub expiry_ns: u64,
    pub period_ns: u64,
    pub state: TimerState,
    pub softirq_enabled: bool,
}

impl Hrtimer {
    pub fn new(id: u64, callback: TimerCallback) -> Self {
        Self {
            id,
            callback,
            expiry_ns: 0,
            period_ns: 0,
            state: TimerState::Inactive,
            softirq_enabled: true,
        }
    }
    
    pub fn start_once(&mut self, delay_ns: u64) {
        self.expiry_ns = get_ktime_get_mono_fast_ns() + delay_ns;
        self.period_ns = 0;
        self.state = TimerState::Running;
    }
    
    pub fn start_periodic(&mut self, initial_delay_ns: u64, period_ns: u64) {
        self.expiry_ns = get_ktime_get_mono_fast_ns() + initial_delay_ns;
        self.period_ns = period_ns;
        self.state = TimerState::Running;
    }
    
    pub fn stop(&mut self) {
        self.state = TimerState::Inactive;
    }
    
    pub fn expires_in_ns(&self) -> u64 {
        let now = get_ktime_get_mono_fast_ns();
        if self.expiry_ns > now {
            self.expiry_ns - now
        } else {
            0
        }
    }
}

// ============================================================================
// TIMER CLOCK MANAGEMENT
// ============================================================================

#[derive(Debug)]
pub struct TimerClock {
    next_timer_id: AtomicU64,
    active_timers: VecDeque<Hrtimer>,
    min_expiry_ns: AtomicU64,
    last_tick_ns: AtomicU64,
    total_fires: AtomicU64,
}

impl TimerClock {
    pub fn new() -> Self {
        Self {
            next_timer_id: AtomicU64::new(1),
            active_timers: VecDeque::new(),
            min_expiry_ns: AtomicU64::new(u64::MAX),
            last_tick_ns: AtomicU64::new(0),
            total_fires: AtomicU64::new(0),
        }
    }
    
    /// Start a one-shot timer
    pub fn start(&mut self, delay_ns: u64) -> u64 {
        let id = self.next_timer_id.fetch_add(1, Ordering::Relaxed);
        
        let mut timer = Hrtimer::new(id, Box::new(|| TimerResult::Expire));
        timer.start_once(delay_ns);
        
        self.active_timers.push_back(timer);
        self.update_min_expiry();
        
        id
    }
    
    /// Start a recurring timer
    pub fn start_periodic(&mut self, initial_delay_ns: u64, period_ns: u64) -> u64 {
        let id = self.next_timer_id.fetch_add(1, Ordering::Relaxed);
        
        let mut timer = Hrtimer::new(id, Box::new(|| TimerResult::RemainRunning));
        timer.start_periodic(initial_delay_ns, period_ns);
        
        self.active_timers.push_back(timer);
        self.update_min_expiry();
        
        id
    }
    
    /// Stop a timer by ID
    pub fn stop(&mut self, id: u64) {
        if let Some(pos) = self.active_timers.iter().position(|t| t.id == id) {
            self.active_timers[pos].state = TimerState::Inactive;
            self.active_timers.remove(pos);
            self.update_min_expiry();
        }
    }
    
    /// Process all expired timers
    pub fn tick(&mut self, current_time_ns: u64) -> Vec<u64> {
        self.last_tick_ns.store(current_time_ns, Ordering::Relaxed);
        
        let mut fired = Vec::new();
        let mut deadtimers = Vec::new();
        
        for timer in &self.active_timers {
            if timer.expiry_ns <= current_time_ns && timer.state == TimerState::Running {
                deadtimers.push(timer.id);
            }
        }
        
        for id in &deadtimers {
            if let Some(timer_idx) = self.active_timers.iter().position(|t| t.id == *id) {
                let timer = self.active_timers.get_mut(timer_idx).unwrap();
                let result = (timer.callback)();
                
                match result {
                    TimerResult::Expire => {
                        timer.stop();
                        fired.push(*id);
                    },
                    TimerResult::RemainRunning => {
                        timer.expiry_ns += timer.period_ns;
                    },
                }
                
                self.total_fires.fetch_add(1, Ordering::Relaxed);
            }
        }
        
        self.update_min_expiry();
        fired.sort();
        fired.dedup();
        fired
    }
    
    /// Get minimum time until next timer expires
    pub fn get_next_expire_delta(&self) -> u64 {
        let min = self.min_expiry_ns.load(Ordering::Relaxed);
        let now = get_ktime_get_mono_fast_ns();
        
        if min > now {
            min - now
        } else {
            0
        }
    }
    
    fn update_min_expiry(&mut self) {
        let min = self.active_timers
            .iter()
            .filter(|t| t.state == TimerState::Running)
            .map(|t| t.expiry_ns)
            .min()
            .unwrap_or(u64::MAX);
        
        self.min_expiry_ns.store(min, Ordering::Relaxed);
    }
}

impl Default for TimerClock {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DELAY ROUTINES AND SLEEP FUNCTIONS
// ============================================================================

pub fn rdtsc() -> u64 {
    unsafe { std::arch::x86_64::_rdtsc() as u64 }
}

pub fn ktime_get_mono_fast_ns() -> u64 {
    tsc_clocksource().ktime_get_mono_fast_ns()
}

pub fn get_ktime_get_mono_fast_ns() -> u64 {
    static CLOCK_SOURCE: once_cell::sync::Lazy<Clocksource> = 
        lazy_static::lazy!(Clocksource::new("global_tsc", ClockSourceType::TSC));
    
    CLOCK_SOURCE.ktime_get_mono_fast_ns()
}

pub fn udelay(microseconds: u32) {
    let start = rdtsc();
    let frequency = unsafe { 
        // Approximate CPU frequency detection
        2_000_000_000 // 2 GHz baseline
    };
    let end = start + (microseconds as u64 * frequency / 1_000_000);
    
    while rdtsc() < end {
        unsafe { std::arch::x86_64::_mm_pause(); }
    }
}

pub fn mdelay(milliseconds: u32) {
    if milliseconds == 0 {
        return;
    }
    
    let us_per_ms = 1000;
    let batches = 256;
    let us_per_batch = (us_per_ms + batches - 1) / batches;
    
    for _ in 0..milliseconds {
        for _ in 0..batches {
            udelay(us_per_batch as u32);
        }
    }
}

pub fn msleep(milliseconds: u32) {
    if milliseconds == 0 {
        return;
    }
    
    // Use scheduler yield instead of busy-waiting
    sched_yield_for_ms(milliseconds);
}

pub fn usleep(usecs: u32) {
    let ms = (usecs + 999) / 1000;
    msleep(ms);
}

fn sched_yield_for_ms(milliseconds: u32) {
    let start = get_ktime_get_mono_fast_ns();
    let deadline = start + (milliseconds as u64 * 1_000_000);
    
    while get_ktime_get_mono_fast_ns() < deadline {
        sched_yield();
    }
}

pub fn sched_yield() {
    // In production, would call into kernel scheduler
    // For now, just yield to other threads
    std::thread::yield_now();
}

// ============================================================================
// NTP CALIBRATION SUPPORT
// ============================================================================

#[derive(Debug)]
pub struct NtpCalibration {
    public_offset_ns: i64,
    clock_adj: ClockadjData,
    max_error_ppm: u32,
}

#[derive(Debug)]
pub struct ClockadjData {
    pub offset_ns: i64,
    pub freq_ppm: i64,
    pub maxerror_ppm: u32,
    pub status: u16,
}

impl NtpCalibration {
    pub fn new() -> Self {
        Self {
            public_offset_ns: 0,
            clock_adj: ClockadjData {
                offset_ns: 0,
                freq_ppm: 0,
                maxerror_ppm: 1000,
                status: 0,
            },
            max_error_ppm: 1000,
        }
    }
    
    pub fn adjust(&mut self, offset_ns: i64) {
        self.public_offset_ns += offset_ns;
        
        // Clamp within max error
        if self.public_offset_ns.abs() > 1_000_000_000 {
            self.public_offset_ns = if self.public_offset_ns > 0 {
                1_000_000_000
            } else {
                -1_000_000_000
            };
        }
    }
    
    pub fn get_offset_ns(&self) -> i64 {
        self.public_offset_ns
    }
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum TimersError {
    InvalidArgument,
    TimeoutExpired,
    OperationNotSupported,
    ResourceBusy,
}

impl std::fmt::Display for TimersError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimersError::InvalidArgument => write!(f, "Invalid argument"),
            TimersError::TimeoutExpired => write!(f, "Timeout expired"),
            TimersError::OperationNotSupported => write!(f, "Operation not supported"),
            TimersError::ResourceBusy => write!(f, "Resource busy"),
        }
    }
}

impl std::error::Error for TimersError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clocksource_creation() {
        let cs = Clocksource::new("TEST", ClockSourceType::TSC);
        assert_eq!(cs.name, "TEST");
        assert!(!cs.running.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_timer_clock_basic() {
        let mut tc = TimerClock::new();
        let id = tc.start(1_000_000); // 1ms in the future
        
        assert_eq!(id, 1);
        assert_eq!(tc.active_timers.len(), 1);
        
        tc.stop(id);
        assert_eq!(tc.active_timers.len(), 0);
    }
    
    #[test]
    fn test_periodic_timer() {
        let mut tc = TimerClock::new();
        let id = tc.start_periodic(100, 1000); // 100µs initial, 1ms period
        
        assert_eq!(id, 1);
        
        // Simulate ticks
        tc.tick(200);
        assert_eq!(tc.active_timers.len(), 1);
    }
    
    #[test]
    fn test_delay_functions() {
        let start = rdtsc();
        mdelay(1); // ~1ms
        let elapsed = rdtsc() - start;
        
        assert!(elapsed > 1000); // Should be at least some ticks
    }
    
    #[test]
    fn test_ntp_calibration() {
        let mut ntp = NtpCalibration::new();
        
        ntp.adjust(100);
        assert_eq!(ntp.get_offset_ns(), 100);
        
        ntp.adjust(-50);
        assert_eq!(ntp.get_offset_ns(), 50);
    }
    
    #[test]
    fn test_timer_expiration() {
        let mut tc = TimerClock::new();
        let id = tc.start(0); // Immediate expiration
        
        let now = get_ktime_get_mono_fast_ns();
        let fired = tc.tick(now);
        
        assert!(fired.contains(&id));
    }
}
