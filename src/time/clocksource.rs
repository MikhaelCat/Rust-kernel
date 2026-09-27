//! Clock Sources and Events for Linux Kernel on Rust
//! Реализация clocksource и clockevent для системы таймеров

// Re-exported from timer.rs - this is a stub for module organization

use crate::time::timer::{Clocksource, ClockSourceType};

// ============================================================================
// CLOCK EVENT CONFIGURATION  
// ============================================================================

pub use crate::time::timer::{Clockevent, ClockeventMode, ClockeventFeatures, TimersError};

/// Get the default clocksource for the system
pub fn get_default_clocksource() -> Clocksource {
    let mut cs = Clocksource::new("tsc", ClockSourceType::TSC);
    cs.running.store(true, std::sync::atomic::Ordering::Relaxed);
    cs.calibrate(1_000_000);
    cs
}

/// Select clocksource at boot
pub fn select_clocksource(name: &str) -> Option<Clocksource> {
    match name {
        "tsc" | "TSC" => {
            let mut cs = Clocksource::new("TSC", ClockSourceType::TSC);
            cs.running.store(true, std::sync::atomic::Ordering::Relaxed);
            Some(cs)
        },
        "hpet" => {
            let cs = Clocksource::new("HPET", ClockSourceType::HPET);
            Some(cs)
        },
        "acpi_pm" => {
            let cs = Clocksource::new("ACPI-PM", ClockSourceType::ACPI_PM);
            Some(cs)
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_select_clocksource_tsc() {
        let cs = select_clocksource("TSC").unwrap();
        assert_eq!(cs.name, "TSC");
    }
    
    #[test]
    fn test_select_invalid_clocksource() {
        let cs = select_clocksource("invalid");
        assert!(cs.is_none());
    }
    
    #[test]
    fn test_get_default() {
        let cs = get_default_clocksource();
        assert_eq!(cs.name, "tsc");
        assert!(cs.running.load(std::sync::atomic::Ordering::Relaxed));
    }
}
