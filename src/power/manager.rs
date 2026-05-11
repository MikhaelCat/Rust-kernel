use super::cpufreq::CpuFreq;
use super::cpuidle::CpuIdle;
use super::error::PowerError;
use super::pm_qos::PmQos;
use super::regulator::Regulator;
use super::suspend::{SuspendManager, SuspendState};
use super::thermal::Thermal;

#[derive(Debug)]
pub struct PowerManager {
    idle: CpuIdle,
    qos: PmQos,
    regulator: Regulator,
    thermal: Thermal,
    suspend: SuspendManager,
    freq_khz: u64,
}

impl PowerManager {
    pub fn new() -> Self {
        Self {
            idle: CpuIdle::default(),
            qos: PmQos::default(),
            regulator: Regulator::default(),
            thermal: Thermal::default(),
            suspend: SuspendManager::new(),
            freq_khz: 0,
        }
    }

    pub fn set_frequency(&mut self, khz: u64) -> Result<CpuFreq, PowerError> {
        if khz == 0 {
            return Err(PowerError::InvalidFrequency);
        }
        self.freq_khz = khz;
        Ok(CpuFreq::set(khz))
    }

    pub fn enter_idle(&mut self) {
        self.idle.enter();
    }

    pub fn set_qos_latency(&mut self, latency_us: u32) {
        self.qos.set_latency(latency_us);
    }

    pub fn enable_regulator(&mut self) {
        self.regulator.enable();
    }

    pub fn set_temp(&mut self, celsius: i32) {
        self.thermal.set_temp(celsius);
    }

    pub fn suspend(&mut self) {
        self.suspend.suspend();
    }

    pub fn resume(&mut self) {
        self.suspend.resume();
    }

    pub fn suspend_state(&self) -> SuspendState {
        self.suspend.state()
    }

    pub fn frequency_khz(&self) -> u64 {
        self.freq_khz
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_lifecycle() {
        let mut p = PowerManager::new();
        p.set_frequency(1_800_000).expect("freq set failed");
        p.enter_idle();
        p.set_qos_latency(50);
        p.enable_regulator();
        p.set_temp(65);
        p.suspend();
        assert_eq!(p.suspend_state(), SuspendState::Suspended);
        p.resume();
        assert_eq!(p.suspend_state(), SuspendState::Active);
        assert_eq!(p.frequency_khz(), 1_800_000);
    }
}
