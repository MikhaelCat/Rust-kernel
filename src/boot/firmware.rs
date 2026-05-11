#[derive(Debug, Default)]
pub struct BootFirmware {
    loaded: bool,
}

impl BootFirmware {
    pub fn load(&mut self) {
        self.loaded = true;
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }

    pub fn unload(&mut self) {
        self.loaded = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn firmware_loads() {
        let mut fw = BootFirmware::default();
        fw.load();
        assert!(fw.loaded());
    }
}
