#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeResult {
    pub matched: bool,
}

pub fn probe_device(vid: u16, did: u16) -> ProbeResult {
    let matched = (vid, did) == (0x8086, 0x100e) || (vid, did) == (0x1022, 0x149c);
    ProbeResult { matched }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_known_devices() {
        assert!(probe_device(0x8086, 0x100e).matched);
        assert!(!probe_device(0xffff, 0xffff).matched);
    }
}
