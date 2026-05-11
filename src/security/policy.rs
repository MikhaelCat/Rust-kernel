#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Deny,
}

pub fn check_mount(is_privileged: bool) -> Decision {
    if is_privileged {
        Decision::Allow
    } else {
        Decision::Deny
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mount_requires_privilege() {
        assert_eq!(check_mount(true), Decision::Allow);
        assert_eq!(check_mount(false), Decision::Deny);
    }
}
