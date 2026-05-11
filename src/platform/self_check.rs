use super::board::Board;
use super::system::PlatformSystem;

pub fn run_platform_self_check() -> bool {
    let mut p = PlatformSystem::new(Board::generic(), &[1, 2, 3, 4]);
    p.bootstrap();
    let s = p.snapshot();
    s.firmware_loaded && s.acpi_loaded && s.efi_runtime && s.compatible && s.policy_pass && s.ready
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_self_check_passes() {
        assert!(run_platform_self_check());
    }
}
