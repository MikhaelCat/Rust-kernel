use super::manager::KernelManager;

#[derive(Default)]
pub struct KernelSystem {
    km: KernelManager,
}

impl KernelSystem {
    pub fn new() -> Self {
        Self {
            km: KernelManager::new(),
        }
    }

    pub fn bootstrap(&mut self) {
        self.km.spawn_process("init", 0);
    }

    pub fn tick(&mut self) -> u32 {
        self.km.tick().expect("tick failed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_bootstrap_tick() {
        let mut k = KernelSystem::new();
        k.bootstrap();
        assert_eq!(k.tick(), 1);
    }
}
