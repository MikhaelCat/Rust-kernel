use super::{PowerError, PowerManager};

#[derive(Debug)]
pub struct PowerSystem {
    mgr: PowerManager,
}

impl PowerSystem {
    pub fn new() -> Self {
        Self {
            mgr: PowerManager::new(),
        }
    }

    pub fn bootstrap(&mut self) -> Result<(), PowerError> {
        self.mgr.set_frequency(1_200_000)?;
        self.mgr.set_qos_latency(100);
        self.mgr.enable_regulator();
        Ok(())
    }

    pub fn manager(&self) -> &PowerManager {
        &self.mgr
    }

    pub fn manager_mut(&mut self) -> &mut PowerManager {
        &mut self.mgr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::power::suspend::SuspendState;

    #[test]
    fn system_bootstrap_and_suspend_cycle() {
        let mut s = PowerSystem::new();
        s.bootstrap().expect("bootstrap failed");
        assert_eq!(s.manager().frequency_khz(), 1_200_000);

        s.manager_mut().suspend();
        assert_eq!(s.manager().suspend_state(), SuspendState::Suspended);
        s.manager_mut().resume();
        assert_eq!(s.manager().suspend_state(), SuspendState::Active);
    }
}
