#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinuxSyscallClass {
    Process,
    File,
    Memory,
    Network,
    Sync,
    Unknown,
}

pub fn linux_syscall_name(num: u64) -> &'static str {
    match num {
        0 => "read",
        1 => "write",
        2 => "open",
        3 => "close",
        9 => "mmap",
        11 => "munmap",
        39 => "getpid",
        41 => "socket",
        42 => "connect",
        44 => "sendto",
        45 => "recvfrom",
        202 => "futex",
        _ => "unknown",
    }
}

pub fn linux_syscall_class(num: u64) -> LinuxSyscallClass {
    match num {
        0 | 1 | 2 | 3 => LinuxSyscallClass::File,
        9 | 11 => LinuxSyscallClass::Memory,
        39 => LinuxSyscallClass::Process,
        41 | 42 | 44 | 45 => LinuxSyscallClass::Network,
        202 => LinuxSyscallClass::Sync,
        _ => LinuxSyscallClass::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_syscalls() {
        assert_eq!(linux_syscall_name(39), "getpid");
        assert_eq!(linux_syscall_class(41), LinuxSyscallClass::Network);
    }
}
