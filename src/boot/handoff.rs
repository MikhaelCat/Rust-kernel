#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Handoff {
    pub entry: u64,
    pub min_memory_bytes: u64,
    pub firmware_loaded: bool,
}

impl Handoff {
    pub fn valid(&self) -> bool {
        self.entry != 0 && self.min_memory_bytes >= 1024 * 1024 && self.firmware_loaded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handoff_validation() {
        assert!(
            Handoff {
                entry: 0x1000,
                min_memory_bytes: 8 * 1024 * 1024,
                firmware_loaded: true
            }
            .valid()
        );
        assert!(
            !Handoff {
                entry: 0,
                min_memory_bytes: 8 * 1024 * 1024,
                firmware_loaded: true
            }
            .valid()
        );
        assert!(
            !Handoff {
                entry: 0x1000,
                min_memory_bytes: 256 * 1024,
                firmware_loaded: true
            }
            .valid()
        );
        assert!(
            !Handoff {
                entry: 0x1000,
                min_memory_bytes: 8 * 1024 * 1024,
                firmware_loaded: false
            }
            .valid()
        );
    }
}
