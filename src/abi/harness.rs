use super::syscall::{AbiKernel, Sysno};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbiComplianceReport {
    pub getpid_ok: bool,
    pub open_ok: bool,
    pub write_ok: bool,
    pub read_ok: bool,
    pub close_ok: bool,
    pub mmap_ok: bool,
    pub munmap_ok: bool,
    pub futex_ok: bool,
    pub socket_ok: bool,
    pub connect_ok: bool,
    pub send_recv_ok: bool,
    pub process_isolation_ok: bool,
    pub exec_wait_ok: bool,
    pub clone_poll_fcntl_ok: bool,
    pub bind_listen_accept_ok: bool,
    pub dup_pipe_ok: bool,
    pub signal_waitpid_ok: bool,
}

impl AbiComplianceReport {
    pub fn all_ok(&self) -> bool {
        self.getpid_ok
            && self.open_ok
            && self.write_ok
            && self.read_ok
            && self.close_ok
            && self.mmap_ok
            && self.munmap_ok
            && self.futex_ok
            && self.socket_ok
            && self.connect_ok
            && self.send_recv_ok
            && self.process_isolation_ok
            && self.exec_wait_ok
            && self.clone_poll_fcntl_ok
            && self.bind_listen_accept_ok
            && self.dup_pipe_ok
            && self.signal_waitpid_ok
    }
}

pub fn run_abi_compliance() -> AbiComplianceReport {
    let mut k = AbiKernel::new(1);

    let pid = k.syscall3(Sysno::Getpid, 0, 0, 0);
    let fd = k.syscall3(Sysno::Open, 123, 0, 0);
    let wr = k.syscall3(Sysno::Write, fd, 0, 8);
    let rd = k.syscall3(Sysno::Read, fd, 0, 8);
    let cl = k.syscall3(Sysno::Close, fd, 0, 0);

    let addr = k.syscall3(Sysno::Mmap, 4096, 0x3, 0x22);
    let unmap = k.syscall3(Sysno::Munmap, addr, 0, 0);

    let futex_wait = k.syscall3(Sysno::Futex, 0, 0, 0);
    let futex_wake = k.syscall3(Sysno::Futex, 0, 1, 1);

    let sock = k.syscall3(Sysno::Socket, 0, 0, 0);
    let conn = k.syscall3(Sysno::Connect, sock, 0, 0);
    let snd = k.syscall3(Sysno::Sendto, sock, 0, 16);
    let rcv = k.syscall3(Sysno::Recvfrom, sock, 0, 16);

    let child = k.syscall3(Sysno::ForkLike, 0, 0, 0);
    let sw = k.syscall3(Sysno::SwitchTask, child, 0, 0);
    let child_fd = k.syscall3(Sysno::Open, 999, 0, 0);
    let back = k.syscall3(Sysno::SwitchTask, 1, 0, 0);
    let parent_read_child_fd = k.syscall3(Sysno::Read, child_fd, 0, 1);

    let sw2 = k.syscall3(Sysno::SwitchTask, child, 0, 0);
    let ex = k.syscall3(Sysno::ExecLike, 0, 0, 0);
    let quit = k.syscall3(Sysno::ExitLike, 0, 0, 0);
    let sw_back = k.syscall3(Sysno::SwitchTask, 1, 0, 0);
    let waited_pid = k.syscall3(Sysno::WaitLike, 0, 0, 0);

    let clone_pid = k.syscall3(Sysno::Clone, 0x500, 0, 0);
    let pol = k.syscall3(Sysno::Poll, 3, 4, 0);
    let fset = k.syscall3(Sysno::Fcntl, 3, 1, 1);
    let fget = k.syscall3(Sysno::Fcntl, 3, 0, 0);

    let ls = k.syscall3(Sysno::SocketEx, 0, 0, 0);
    let b = k.syscall3(Sysno::Bind, ls, 0, 0);
    let l = k.syscall3(Sysno::Listen, ls, 0, 0);
    let a = k.syscall3(Sysno::Accept, ls, 0, 0);

    let d = k.syscall3(Sysno::Dup, 3, 0, 0);
    let d2 = k.syscall3(Sysno::Dup2, 3, 10, 0);
    let wpipe = k.syscall3(Sysno::Pipe, 0, 0, 0);
    let (pw, pr) = k.syscall_pipe_rw(wpipe, wpipe - 1, b'Z');

    let child2 = k.syscall3(Sysno::ForkLike, 0, 0, 0);
    let _ = k.syscall3(Sysno::SwitchTask, child2, 0, 0);
    let _ = k.syscall3(Sysno::ExitLike, 0, 0, 0);
    let _ = k.syscall3(Sysno::SwitchTask, 1, 0, 0);
    let kill = k.syscall3(Sysno::KillLike, child2, 15, 0);
    let pnd = k.pending_signals(child2);
    let wp = k.syscall3(Sysno::WaitpidLike, child2, 0, 0);

    AbiComplianceReport {
        getpid_ok: pid == 1,
        open_ok: fd >= 3,
        write_ok: wr == 8,
        read_ok: rd == 8,
        close_ok: cl == 0,
        mmap_ok: addr > 0,
        munmap_ok: unmap == 0,
        futex_ok: futex_wait == 0 && futex_wake == 1,
        socket_ok: sock >= 100,
        connect_ok: conn == 0,
        send_recv_ok: snd == 16 && rcv == 16,
        process_isolation_ok: sw == 0 && back == 0 && child_fd >= 3 && parent_read_child_fd < 0,
        exec_wait_ok: sw2 == 0 && ex == 0 && quit == 0 && sw_back == 0 && waited_pid == child,
        clone_poll_fcntl_ok: clone_pid >= 2 && pol >= 1 && fset == 0 && fget == 1,
        bind_listen_accept_ok: b == 0 && l == 0 && a >= 200,
        dup_pipe_ok: d >= 300 && d2 == 10 && pw == 1 && pr == b'Z' as i64,
        signal_waitpid_ok: kill == 0 && pnd >= 1 && wp == child2,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compliance_passes() {
        let r = run_abi_compliance();
        assert!(r.all_ok());
    }
}
