#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirmwareMeta {
    pub vendor: String,
    pub version: String,
}

#[derive(Debug, Default)]
pub struct Firmware {
    loaded: bool,
    meta: Option<FirmwareMeta>,
}

impl Firmware {
    pub fn load(&mut self) {
        self.loaded = true;
        self.meta = Some(FirmwareMeta {
            vendor: "rust-linux".to_string(),
            version: "0.1".to_string(),
        });
    }

    pub fn loaded(&self) -> bool {
        self.loaded
    }

    pub fn meta(&self) -> Option<&FirmwareMeta> {
        self.meta.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fw_load_and_meta() {
        let mut f = Firmware::default();
        f.load();
        assert!(f.loaded());
        assert_eq!(f.meta().map(|m| m.vendor.as_str()), Some("rust-linux"));
    }
}
