use super::clockevent::ClockEvent;
use super::clocksource::ClockSource;
use super::error::TimeError;
use super::hrtimer::HrTimer;
use super::ntp::NtpAdjust;
use super::tick::Tick;
use super::timerfd::TimerFd;

#[derive(Debug, Default)]
pub struct TimeManager {
    tick: Tick,
    hrtimer: HrTimer,
    timerfd: TimerFd,
    event: ClockEvent,
    source: Option<ClockSource>,
    ntp_ppm: i32,
}

impl TimeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_clocksource(&mut self, src: ClockSource) {
        self.source = Some(src);
    }

    pub fn tick(&mut self) {
        self.tick.inc();
        self.event.fire();
        self.timerfd.tick();
    }

    pub fn advance_ns(&mut self, ns: u64) {
        self.hrtimer.advance_ns(ns);
    }

    pub fn adjust_ntp(&mut self, adj: NtpAdjust) -> Result<(), TimeError> {
        if adj.ppm.abs() > 500_000 {
            return Err(TimeError::InvalidAdjustment);
        }
        self.ntp_ppm = adj.ppm;
        Ok(())
    }

    pub fn jiffies(&self) -> u64 {
        self.tick.jiffies()
    }

    pub fn now_ns(&self) -> u64 {
        self.hrtimer.now_ns()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_flow() {
        let mut t = TimeManager::new();
        t.set_clocksource(ClockSource::monotonic());
        t.tick();
        t.advance_ns(1000);
        t.adjust_ntp(NtpAdjust::new(120))
            .expect("ntp adjust failed");
        assert_eq!(t.jiffies(), 1);
        assert_eq!(t.now_ns(), 1000);
    }
}
