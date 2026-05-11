use super::params::BootParams;

pub fn validate_boot_params(params: &BootParams) -> bool {
    params.validate() && params.get("console").is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::boot::params::BootParams;

    #[test]
    fn requires_console_arg() {
        assert!(validate_boot_params(&BootParams::new(
            "console=ttyS0",
            false
        )));
        assert!(!validate_boot_params(&BootParams::new(
            "root=/dev/ram0",
            false
        )));
    }
}
