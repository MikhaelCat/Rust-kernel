#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemHealth {
    pub boot_ok: bool,
    pub arch_ok: bool,
    pub kernel_ok: bool,
    pub net_ok: bool,
    pub ipc_ok: bool,
    pub security_ok: bool,
}

impl SystemHealth {
    pub fn all_ok(&self) -> bool {
        self.boot_ok
            && self.arch_ok
            && self.kernel_ok
            && self.net_ok
            && self.ipc_ok
            && self.security_ok
    }
}
