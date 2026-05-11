use super::clone::CloneFlags;
use super::errno::{EBADF, ECHILD, EINVAL, ENOTCONN};
use super::execve::ExecImage;
use super::fcntl::FcntlTable;
use super::fd_ops::FdOps;
use super::memory::MmapTable;
use super::memory_flags::{MmapFlags, MmapProt};
use super::net::SocketTable;
use super::net_ext::NetExt;
use super::poll::{PollFd, poll_like};
use super::process_semantics::ProcSemantics;
use super::process_table::ProcessTable;
use super::signal::{Signal, SignalTable};
use super::sync::FutexTable;
use super::wait::make_wait_status;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sysno {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Poll = 7,
    Mmap = 9,
    Munmap = 11,
    Pipe = 22,
    Dup = 32,
    Dup2 = 33,
    Getpid = 39,
    Socket = 41,
    Connect = 42,
    Accept = 43,
    Sendto = 44,
    Recvfrom = 45,
    Bind = 49,
    Listen = 50,
    Clone = 56,
    Fcntl = 72,
    Futex = 202,
    KillLike = 10006,
    WaitpidLike = 10007,
    ForkLike = 10000,
    SwitchTask = 10001,
    ExecLike = 10002,
    ExitLike = 10003,
    WaitLike = 10004,
    SocketEx = 10005,
}

#[derive(Debug)]
pub struct AbiKernel {
    procs: ProcessTable,
    proc_sem: ProcSemantics,
    mmap: MmapTable,
    futex: FutexTable,
    net: SocketTable,
    net_ext: NetExt,
    fcntl: FcntlTable,
    fd_ops: FdOps,
    sigtab: SignalTable,
    last_exited: Option<i64>,
}

impl AbiKernel {
    pub fn new(pid: i64) -> Self {
        Self {
            procs: ProcessTable::new(pid),
            proc_sem: ProcSemantics::new(pid),
            mmap: MmapTable::new(),
            futex: FutexTable::default(),
            net: SocketTable::new(),
            net_ext: NetExt::new(),
            fcntl: FcntlTable::default(),
            fd_ops: FdOps::new(),
            sigtab: SignalTable::default(),
            last_exited: None,
        }
    }

    pub fn syscall3(&mut self, no: Sysno, a0: i64, a1: i64, a2: i64) -> i64 {
        match no {
            Sysno::Getpid => self.procs.current_pid(),
            Sysno::ForkLike => {
                let child = self.procs.fork_like();
                let _ = self.proc_sem.fork_like();
                child
            }
            Sysno::Clone => {
                let _flags = CloneFlags::from_bits(a0 as u64);
                let child = self.procs.fork_like();
                let _ = self.proc_sem.fork_like();
                child
            }
            Sysno::SwitchTask => {
                let ok1 = self.procs.switch_to(a0);
                let ok2 = self.proc_sem.switch_to(a0);
                if ok1 && ok2 { 0 } else { EINVAL }
            }
            Sysno::ExecLike => {
                let img = ExecImage::new("/bin/exec", &["exec"], &["PATH=/bin"]);
                if img.sane() && self.proc_sem.exec_like(&img.path) {
                    0
                } else {
                    EINVAL
                }
            }
            Sysno::ExitLike => {
                if self.proc_sem.exit_like(a0) {
                    self.last_exited = Some(self.procs.current_pid());
                    0
                } else {
                    EINVAL
                }
            }
            Sysno::WaitLike => match self.proc_sem.wait_like() {
                Some((pid, code)) => {
                    let _status = make_wait_status(code);
                    pid
                }
                None => EINVAL,
            },
            Sysno::WaitpidLike => match self.proc_sem.waitpid_like(a0) {
                Some((pid, _)) => pid,
                None => ECHILD,
            },
            Sysno::KillLike => {
                if !self.proc_sem.has_pid(a0) {
                    return EINVAL;
                }
                let sig = if a1 == 9 { Signal::Kill } else { Signal::Term };
                self.sigtab.send(a0, sig)
            }
            Sysno::Open => {
                let _flags = a1;
                let _mode = a2;
                let pseudo_path = format!("fd://{}", a0);
                self.procs.file_table_mut().open(&pseudo_path)
            }
            Sysno::Close => self.procs.file_table_mut().close(a0),
            Sysno::Write => self.procs.file_table_mut().write(a0, a2 as usize),
            Sysno::Read => self.procs.file_table_mut().read(a0, a2 as usize),
            Sysno::Mmap => {
                let _prot = MmapProt::from_bits(a1 as u64);
                let _flags = MmapFlags::from_bits(a2 as u64);
                self.mmap.mmap(a0 as usize)
            }
            Sysno::Munmap => self.mmap.munmap(a0),
            Sysno::Futex => {
                if a1 == 0 {
                    self.futex.futex_wait()
                } else {
                    self.futex.futex_wake(a2)
                }
            }
            Sysno::Socket => self.net.socket(),
            Sysno::SocketEx => self.net_ext.socket(),
            Sysno::Connect => self.net.connect(a0),
            Sysno::Sendto => self.net.sendto(a0, a2 as usize),
            Sysno::Recvfrom => self.net.recvfrom(a0, a2 as usize),
            Sysno::Bind => self.net_ext.bind(a0),
            Sysno::Listen => self.net_ext.listen(a0),
            Sysno::Accept => self.net_ext.accept(a0),
            Sysno::Poll => {
                let fds = [
                    PollFd {
                        fd: a0,
                        readable: true,
                        writable: false,
                    },
                    PollFd {
                        fd: a1,
                        readable: false,
                        writable: a1 >= 0,
                    },
                ];
                poll_like(&fds, a2 as i32)
            }
            Sysno::Fcntl => {
                if a1 == 1 {
                    self.fcntl.set_cloexec(a0, a2 != 0)
                } else {
                    self.fcntl.get_cloexec(a0)
                }
            }
            Sysno::Dup => self.fd_ops.dup(a0),
            Sysno::Dup2 => self.fd_ops.dup2(a0, a1),
            Sysno::Pipe => {
                let (_r, w) = self.fd_ops.pipe();
                w
            }
        }
    }

    pub fn syscall_pipe_rw(&mut self, wfd: i64, rfd: i64, byte: u8) -> (i64, i64) {
        let w = self.fd_ops.pipe_write(wfd, byte);
        let r = self.fd_ops.pipe_read(rfd);
        (w, r)
    }

    pub fn pending_signals(&self, pid: i64) -> usize {
        self.sigtab.pending_count(pid)
    }

    pub fn errno_sanity() -> (i64, i64, i64) {
        (EBADF, EINVAL, ENOTCONN)
    }
}
