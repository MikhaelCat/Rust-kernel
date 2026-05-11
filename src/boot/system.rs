use super::memory_map::MemoryMap;
use super::{BootManager, BootParams};

pub fn boot_smoke(cmdline: &str, image: &[u8]) -> bool {
    let params = BootParams::new(cmdline, false);
    let mut map = MemoryMap::default();
    map.add_region(0, 1024 * 1024);

    let mut mgr = BootManager::new(params, map);
    mgr.boot(Some(image)).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_smoke_ok() {
        let mut img = vec![0u8; 64];
        img[0..4].copy_from_slice(&[0x7f, b'E', b'L', b'F']);
        img[24..32].copy_from_slice(&0x1000u64.to_le_bytes());
        assert!(boot_smoke("console=ttyS0", &img));
    }
}
