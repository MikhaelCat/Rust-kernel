//! Power Management System for Linux Kernel

#[derive(Debug, Clone)]
pub struct CpuFreq {
    pub current_freq_mhz: u32,
    pub min_freq: u32,
    pub max_freq: u32,
    pub scaling_driver: String,
}

impl Default for CpuFreq {
    fn default() -> Self {
        Self {
            current_freq_mhz: 1000,
            min_freq: 800,
            max_freq: 3500,
            scaling_driver: "ondemand".to_string(),
        }
    }
}

impl CpuFreq {
    pub fn set_frequency(&mut self, freq_mhz: u32) -> bool {
        if freq_mhz >= self.min_freq && freq_mhz <= self.max_freq {
            self.current_freq_mhz = freq_mhz;
            true
        } else {
            false
        }
    }

    pub fn get_frequency(&self) -> u32 {
        self.current_freq_mhz
    }
}

#[derive(Debug, Clone)]
pub enum IdleState {
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
}

#[derive(Debug)]
pub struct Cpuidle {
    pub current_state: IdleState,
    pub states_count: usize,
}

impl Default for Cpuidle {
    fn default() -> Self {
        Self::new(6)
    }
}

impl Cpuidle {
    pub fn new(states: usize) -> Self {
        Self {
            current_state: IdleState::C1,
            states_count: states,
        }
    }

    pub fn enter_state(&mut self, state: &IdleState) {
        self.current_state = state.clone();
    }

    pub fn get_current_state(&self) -> &IdleState {
        &self.current_state
    }
}

#[derive(Debug)]
pub struct ThermalZone {
    pub temperature_celsius: u16,
    pub threshold_critical: u16,
    pub threshold_warning: u16,
    pub fans_running: bool,
}

impl Default for ThermalZone {
    fn default() -> Self {
        Self {
            temperature_celsius: 45,
            threshold_critical: 95,
            threshold_warning: 80,
            fans_running: false,
        }
    }
}

impl ThermalZone {
    pub fn update_temperature(&mut self, temp: u16) {
        self.temperature_celsius = temp;
        
        // Fan control
        if temp > self.threshold_warning {
            self.fans_running = true;
        } else if temp < (self.threshold_warning / 2) {
            self.fans_running = false;
        }
    }

    pub fn is_critical(&self) -> bool {
        self.temperature_celsius >= self.threshold_critical
    }

    pub fn is_warning(&self) -> bool {
        self.temperature_celsius >= self.threshold_warning
    }
}
