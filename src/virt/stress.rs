use super::manager::VirtManager;

pub fn run_fleet_stress(count: u32) -> bool {
    let mut m = VirtManager::default();
    for i in 0..count {
        let id = m.create_vm(256 + i, [0, 1, 2, 3, 4, i as u8], 100_000 + i as u64);
        if !m.boot_vm(id) {
            return false;
        }
        let _ = m.balloon_vm(id, 32);
    }
    m.running_count() as u32 == count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fleet_stress_ok() {
        assert!(run_fleet_stress(16));
    }
}
