//! Time/Timer System - High-resolution timers and clock infrastructure
//! 
//! Реализация системы таймеров ядра Linux включая:
//! - High-Resolution Timers (hrtimers)
//! - Clocksource/Clockevent infrastructure  
//! - Delay routines and sleep functions
//! - NTP calibration support

pub mod timer;
pub mod clocksource;
pub mod system;

// Re-export commonly used types
pub use crate::time::timer::{
    Clocksource, ClockSourceType, TimerState, TimerResult, Hrtimer, TimerClock,
    TimersError, Clockevent, ClockeventMode, ClockeventFeatures,
    udelay, mdelay, msleep, usleep, rdtsc, sched_yield, ktime_get_mono_fast_ns,
    NtpCalibration, get_ktime_get_mono_fast_ns,
};

pub use crate::time::clocksource::{get_default_clocksource, select_clocksource};

/// Time subsystem statistics
#[derive(Debug, Clone, Default)]
pub struct TimeStats {
    pub timers_total: u64,
    pub timers_active: u64,
    pub interrupts_total: u64,
}

use std::sync::atomic::AtomicBool;

/// Global timer manager singleton
static TIMER_MANAGER: AtomicBool = AtomicBool::new(false);

/// Time system interface
#[derive(Debug, Clone)]
pub struct TimeSystem {
    pub initialized: bool,
}

/// Initialize timer system
pub fn init_timer_system() -> Result<(), TimersError> {
    // In production, would initialize all CPU timers
    TIMER_MANAGER.store(true, std::sync::atomic::Ordering::SeqCst);
    Ok(())
}

/// Check if timer system is initialized
pub fn is_initialized() -> bool {
    TIMER_MANAGER.load(std::sync::atomic::Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_init_shutdown() {
        assert!(!is_initialized());
        init_timer_system().unwrap();
        assert!(is_initialized());
    }
    
    #[test]
    fn test_delay_timing() {
        let start = rdtsc();
        udelay(100); // 100 microseconds
        let elapsed = rdtsc() - start;
        
        // Should have executed some instructions
        assert!(elapsed > 0);
    }
}
