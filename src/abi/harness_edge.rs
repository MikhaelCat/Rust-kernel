use super::syscall::{AbiKernel, Sysno};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiEdgeReport {
    pub bad_close_ebadf: bool,
    pub send_before_connect_enotconn: bool,
    pub zero_mmap_einval: bool,
    pub bad_munmap_einval: bool,
}

impl AbiEdgeReport {
    pub fn all_ok(&self) -> bool {
        self.bad_close_ebadf
            && self.send_before_connect_enotconn
            && self.zero_mmap_einval
            && self.bad_munmap_einval
    }
}

pub fn run_abi_edge_checks() -> AbiEdgeReport {
    let mut k = AbiKernel::new(1);

    let bad_close = k.syscall3(Sysno::Close, 999, 0, 0);

    let sock = k.syscall3(Sysno::Socket, 0, 0, 0);
    let send_not_conn = k.syscall3(Sysno::Sendto, sock, 0, 8);

    let zero_mmap = k.syscall3(Sysno::Mmap, 0, 0, 0);
    let bad_munmap = k.syscall3(Sysno::Munmap, 0x5555_5555, 0, 0);

    AbiEdgeReport {
        bad_close_ebadf: bad_close == -9,
        send_before_connect_enotconn: send_not_conn == -107,
        zero_mmap_einval: zero_mmap == -22,
        bad_munmap_einval: bad_munmap == -22,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_checks_pass() {
        assert!(run_abi_edge_checks().all_ok());
    }
}
