use super::early_console::EarlyConsole;
use super::error::BootError;
use super::firmware::BootFirmware;
use super::handoff::Handoff;
use super::loader::BootLoader;
use super::memory_map::MemoryMap;
use super::params::BootParams;

#[derive(Debug)]
pub struct BootManager {
    pub params: BootParams,
    pub memory_map: MemoryMap,
    pub console: EarlyConsole,
    pub loader: BootLoader,
    pub firmware: BootFirmware,
    pub log: Vec<&'static str>,
}

impl BootManager {
    pub fn new(params: BootParams, memory_map: MemoryMap) -> Self {
        Self {
            params,
            memory_map,
            console: EarlyConsole::default(),
            loader: BootLoader::new(),
            firmware: BootFirmware::default(),
            log: Vec::new(),
        }
    }

    pub fn boot(&mut self, kernel_image: Option<&[u8]>) -> Result<(), BootError> {
        if !self.params.validate() {
            return Err(BootError::InvalidCmdline);
        }
        if !self.memory_map.is_valid() {
            return Err(BootError::InvalidMemoryMap);
        }

        self.console.write("boot:start");
        self.log.push("params_ok");

        if self.params.get("console").is_some() {
            self.log.push("console_configured");
        }

        let image = kernel_image.ok_or(BootError::MissingKernelImage)?;
        self.loader.load_image(image)?;
        self.log.push("image_loaded");

        self.firmware.load();
        let handoff = Handoff {
            entry: self.loader.entry().0,
            min_memory_bytes: self.memory_map.total_size(),
            firmware_loaded: self.firmware.loaded(),
        };
        if !handoff.valid() {
            self.firmware.unload();
            return Err(BootError::InvalidElfImage);
        }
        self.log.push("handoff_validated");

        self.console.write("boot:ready");
        self.log.push("boot_ready");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_boot_happy_path() {
        let params = BootParams::new("console=ttyS0", false);
        let mut map = MemoryMap::default();
        map.add_region(0, 1024 * 1024);

        let mut img = vec![0u8; 64];
        img[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        img[24..32].copy_from_slice(&0x100000u64.to_le_bytes());

        let mut mgr = BootManager::new(params, map);
        mgr.boot(Some(&img)).expect("boot failed");

        assert_eq!(
            mgr.log,
            vec![
                "params_ok",
                "console_configured",
                "image_loaded",
                "handoff_validated",
                "boot_ready"
            ]
        );
        assert!(mgr.console.bytes_written() > 0);
    }

    #[test]
    fn boot_fails_without_image() {
        let params = BootParams::new("root=/dev/ram0", false);
        let mut map = MemoryMap::default();
        map.add_region(0, 4096);
        let mut mgr = BootManager::new(params, map);

        assert_eq!(mgr.boot(None), Err(BootError::MissingKernelImage));
    }
}
