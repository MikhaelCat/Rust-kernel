//! Boot Manager Implementation for Linux Kernel

#[derive(Debug)]
pub struct BootManager {
    pub cmdline: String,
    pub initrd_loaded: bool,
    pub kernel_modules: Vec<String>,
    pub boot_stage: BootStage,
}

#[derive(Debug, Clone)]
pub enum BootStage {
    PreInit,
    KernelLoad,
    InitrdLoad,
    DriversLoad,
    RootfsMount,
    UserSpace,
    Ready,
}

impl Default for BootManager {
    fn default() -> Self {
        Self::new()
    }
}

impl BootManager {
    pub fn new() -> Self {
        Self {
            cmdline: "".to_string(),
            initrd_loaded: false,
            kernel_modules: Vec::new(),
            boot_stage: BootStage::PreInit,
        }
    }

    pub fn set_cmdline(&mut self, cmdline: &str) {
        self.cmdline = cmdline.to_string();
    }

    pub fn load_initrd(&mut self) -> Result<(), &'static str> {
        if self.cmdline.is_empty() {
            return Err("No initrd specified");
        }
        self.initrd_loaded = true;
        Ok(())
    }

    pub fn add_module(&mut self, name: &str) {
        self.kernel_modules.push(name.to_string());
    }

    pub fn transition_stage(&mut self, stage: BootStage) {
        self.boot_stage = stage;
    }

    pub fn get_stage(&self) -> &BootStage {
        &self.boot_stage
    }

    pub fn is_ready(&self) -> bool {
        matches!(self.boot_stage, BootStage::Ready)
    }
}
