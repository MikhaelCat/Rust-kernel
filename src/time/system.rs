use super::{TimeError, TimeManager, clocksource::ClockSource, ntp::NtpAdjust};

#[derive(Debug, Default)]
pub struct TimeSystem {
    mgr: TimeManager,
}

impl TimeSystem {
    pub fn new() -> Self {
        Self {
            mgr: TimeManager::new(),
        }
    }

    pub fn bootstrap(&mut self) -> Result<(), TimeError> {
        self.mgr.set_clocksource(ClockSource::monotonic());
        self.mgr.adjust_ntp(NtpAdjust::new(0))?;
        Ok(())
    }

    pub fn run_tick(&mut self) {
        self.mgr.tick();
    }

    pub fn advance_ns(&mut self, ns: u64) {
        self.mgr.advance_ns(ns);
    }

    pub fn manager(&self) -> &TimeManager {
        &self.mgr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_tick_flow() {
        let mut t = TimeSystem::new();
        t.bootstrap().expect("bootstrap failed");
        t.run_tick();
        t.advance_ns(500);
        assert_eq!(t.manager().jiffies(), 1);
        assert_eq!(t.manager().now_ns(), 500);
    }
}
