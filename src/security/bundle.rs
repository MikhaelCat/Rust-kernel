use super::apparmor::check_path;
use super::landlock::check_write;
use super::seccomp::filter_syscall;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityBundleReport {
    pub path_ok: bool,
    pub write_ok: bool,
    pub syscall_ok: bool,
}

impl SecurityBundleReport {
    pub fn all_ok(&self) -> bool {
        self.path_ok && self.write_ok && self.syscall_ok
    }
}

pub fn run_security_bundle() -> SecurityBundleReport {
    let path_ok = matches!(
        check_path("/home/user/file"),
        super::apparmor::AaDecision::Allow
    );
    let write_ok = matches!(
        check_write("/tmp/a"),
        super::landlock::LandlockDecision::Allow
    );
    let syscall_ok = matches!(filter_syscall(1), super::seccomp::SeccompAction::Allow);
    SecurityBundleReport {
        path_ok,
        write_ok,
        syscall_ok,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_passes() {
        assert!(run_security_bundle().all_ok());
    }
}
