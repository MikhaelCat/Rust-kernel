//! Syscall Interface for Linux Kernel on Rust
//! Реализация системных вызовов ядра Linux на x86_64

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// ============================================================================
// SYSCALL NUMBERS (x86_64)
// ============================================================================

pub const SYS_READ: u64 = 0;
pub const SYS_WRITE: u64 = 1;
pub const SYS_OPEN: u64 = 2;
pub const SYS_CLOSE: u64 = 3;
pub const SYS_STAT: u64 = 4;
pub const SYS_FSTAT: u64 = 5;
pub const SYS_MMAP: u64 = 9;
pub const SYS_MUNMAP: u64 = 11;
pub const SYS_BRK: u64 = 12;
pub const SYS_IOCTL: u64 = 16;
pub const SYS_PREAD64: u64 = 17;
pub const SYS_PWRITE64: u64 = 18;
pub const SYS_READV: u64 = 19;
pub const SYS_WRITEV: u64 = 20;
pub const SYS_ACCESS: u64 = 21;
pub const SYS_PIPE: u64 = 22;
pub const SYS_SELECT: u64 = 23;
pub const SYS_POLL: u64 = 29;
pub const SYS_MCLAZY: u64 = 35;
pub const SYS_MPROTECT: u64 = 10;
pub const SYS_EXECVE: u64 = 59;
pub const SYS_EXIT: u64 = 60;
pub const SYS_FORK: u64 = 57;
pub const SYS_WAIT4: u64 = 61;
pub const SYS_KILL: u64 = 37;
pub const SYS_UNAME: u64 = 63;
pub const SYS_FCHOWN: u64 = 92;
pub const SYS_CHMOD: u64 = 90;
pub const SYS_GETPID: u64 = 39;
pub const SYS_SETSID: u64 = 215;
pub const SYS_CLONE: u64 = 56;
pub const SYS_DUP2: u64 = 32;
pub const SYS_FCNTL: u64 = 25;
pub const SYS_SETRID: u64 = 113;
pub const SYS_GETTID: u64 = 186;
pub const SYS_SET_TID: u64 = 208;
pub const SYS_EXIT_GROUP: u64 = 235;
pub const SYS_EPOLL_CREATE1: u64 = 281;
pub const SYS_EPOLL_CTL: u64 = 282;
pub const SYS_EPOLL_WAIT: u64 = 283;

// ============================================================================
// SYSCALL DISPATCHER
// ============================================================================

#[derive(Debug)]
pub struct SyscallHandler {
    pub handler: fn(args: &[u64]) -> Result<u64, SyscallError>,
    pub name: &'static str,
}

#[derive(Debug)]
pub struct SyscallDispatcher {
    handlers: HashMap<u64, SyscallHandler>,
    running: AtomicBool,
}

impl Default for SyscallDispatcher {
    fn default() -> Self {
        Self {
            handlers: HashMap::new(),
            running: AtomicBool::new(false),
        }
    }
}

impl SyscallDispatcher {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Register a syscall handler
    pub fn register(&mut self, num: u64, handler: SyscallHandler) {
        self.handlers.insert(num, handler);
    }
    
    /// Invoke syscall
    pub fn invoke(&self, num: u64, args: &[u64]) -> Result<u64, SyscallError> {
        if !self.running.load(Ordering::Relaxed) {
            return Err(SyscallError::NotInitialized);
        }
        
        let handler = self.handlers.get(&num).ok_or_else(|| {
            SyscallError::UnknownSyscall(num)
        })?;
        
        (handler.handler)(args)
    }
    
    pub fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
    }
    
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }
}

// ============================================================================
// REGISTERED SYSCALLS
// ============================================================================

fn init_syscall_dispatcher(dispatcher: &mut SyscallDispatcher) {
    // Process management syscalls
    dispatcher.register(SYS_EXIT, SyscallHandler {
        name: "exit",
        handler: |args| {
            let status = args[0] as i32;
            // In production would exit process with this status
            println!("Process exiting with status {}", status);
            Err(SyscallError::ProcessExited(status as i64))
        },
    });
    
    dispatcher.register(SYS_FORK, SyscallHandler {
        name: "fork",
        handler: |_args| {
            // Return PID 0 to child, PID > 0 to parent
            Ok(1)
        },
    });
    
    dispatcher.register(SYS_EXECVE, SyscallHandler {
        name: "execve",
        handler: |args| {
            let path = unsafe {
                std::ffi::CStr::from_ptr(args[0] as *const i8)
                    .to_string_lossy()
                    .to_string()
            };
            
            println!("Executing: {}", path);
            // Would replace current process image in production
            Ok(0)
        },
    });
    
    dispatcher.register(SYS_EXIT_GROUP, SyscallHandler {
        name: "exit_group",
        handler: |args| {
            let status = args[0] as i32;
            println!("Exiting all threads with status {}", status);
            Err(SyscallError::ProcessExited(status as i64))
        },
    });
    
    // File I/O syscalls
    dispatcher.register(SYS_READ, SyscallHandler {
        name: "read",
        handler: |args| {
            let fd = args[0] as i32;
            let buf_ptr = args[1] as *mut u8;
            let count = args[2] as usize;
            
            // Simple stdout/stdin simulation
            if fd == 0 {
                // stdin - just return success
                Ok(0)
            } else {
                Err(SyscallError::InvalidFd(fd))
            }
        },
    });
    
    dispatcher.register(SYS_WRITE, SyscallHandler {
        name: "write",
        handler: |args| {
            let fd = args[0] as i32;
            let buf_ptr = args[1] as *const u8;
            let count = args[2] as usize;
            
            if fd == 1 || fd == 2 {
                // stdout or stderr - print to console
                unsafe {
                    let bytes = std::slice::from_raw_parts(buf_ptr, count);
                    if fd == 1 {
                        print!("{}", String::from_utf8_lossy(bytes));
                    } else {
                        eprintln!("{}", String::from_utf8_lossy(bytes));
                    }
                }
                Ok(count as u64)
            } else {
                Err(SyscallError::InvalidFd(fd))
            }
        },
    });
    
    dispatcher.register(SYS_OPEN, SyscallHandler {
        name: "open",
        handler: |args| {
            let path = unsafe {
                std::ffi::CStr::from_ptr(args[0] as *const i8)
                    .to_string_lossy()
                    .to_string()
            };
            let flags = args[1] as i32;
            
            println!("Opening file: {} (flags={})", path, flags);
            // In production would call actual open system call
            Err(SyscallError::FileNotFound(format!(
                "Virtual file '{}' not found", path
            )))
        },
    });
    
    dispatcher.register(SYS_CLOSE, SyscallHandler {
        name: "close",
        handler: |args| {
            let fd = args[0] as i32;
            // Close file descriptor
            Ok(0)
        },
    });
    
    // Memory management
    dispatcher.register(SYS_MMAP, SyscallHandler {
        name: "mmap",
        handler: |args| {
            let addr = args[0] as *mut libc::c_void;
            let length = args[1] as usize;
            let prot = args[2] as i32;
            let flags = args[3] as i32;
            let fd = args[4] as i32;
            let offset = args[5] as usize;
            
            // Simulate memory mapping
            let ptr = unsafe { libc::malloc(length as usize) };
            Ok(ptr as u64)
        },
    });
    
    dispatcher.register(SYS_MUNMAP, SyscallHandler {
        name: "munmap",
        handler: |args| {
            let addr = args[0] as *mut libc::c_void;
            let length = args[1] as usize;
            
            unsafe {
                libc::free(addr);
            }
            
            Ok(0)
        },
    });
    
    // Process control
    dispatcher.register(SYS_GETPID, SyscallHandler {
        name: "getpid",
        handler: |_args| {
            // Return fake PID
            Ok(1)
        },
    });
    
    dispatcher.register(SYS_GETTID, SyscallHandler {
        name: "gettid",
        handler: |_args| {
            Ok(1)
        },
    });
    
    dispatcher.register(SYS_FLOCK, SyscallHandler {
        name: "flock",
        handler: |args| {
            let fd = args[0] as i32;
            let op = args[1] as i32;
            
            // Simplified flock implementation
            Ok(0)
        },
    });
    
    dispatcher.start();
}

// ============================================================================
// ERROR TYPES
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum SyscallError {
    Success,
    UnknownSyscall(u64),
    InvalidArgument,
    PermissionDenied,
    NoMemory,
    BadAddress,
    AlreadyExists,
    NotFound,
    FileNotFound(String),
    DirectoryNotEmpty,
    IsADirectory,
    NotADirectory,
    InvalidFd(i32),
    NotEmpty,
    ResourceBusy,
    OperationInProgress,
    OperationAlreadyCompleted,
    TooManyOpenFiles,
    ReadOnlyFilesystem,
    InterruptedBySignal,
    ProcessExited(i64),
    NotInitialized,
}

impl std::fmt::Display for SyscallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyscallError::Success => write!(f, "Success"),
            SyscallError::UnknownSyscall(num) => write!(f, "Unknown syscall {}", num),
            SyscallError::InvalidArgument => write!(f, "Invalid argument"),
            SyscallError::PermissionDenied => write!(f, "Permission denied"),
            SyscallError::NoMemory => write!(f, "Out of memory"),
            SyscallError::BadAddress => write!(f, "Bad address"),
            SyscallError::AlreadyExists => write!(f, "Already exists"),
            SyscallError::NotFound => write!(f, "Not found"),
            SyscallError::FileNotFound(path) => write!(f, "File not found: {}", path),
            SyscallError::DirectoryNotEmpty => write!(f, "Directory not empty"),
            SyscallError::IsADirectory => write!(f, "Is a directory"),
            SyscallError::NotADirectory => write!(f, "Not a directory"),
            SyscallError::InvalidFd(fd) => write!(f, "Invalid file descriptor {}", fd),
            SyscallError::NotEmpty => write!(f, "Directory not empty"),
            SyscallError::ResourceBusy => write!(f, "Resource busy"),
            SyscallError::OperationInProgress => write!(f, "Operation in progress"),
            SyscallError::OperationAlreadyCompleted => write!(f, "Operation already completed"),
            SyscallError::TooManyOpenFiles => write!(f, "Too many open files"),
            SyscallError::ReadOnlyFilesystem => write!(f, "Read-only filesystem"),
            SyscallError::InterruptedBySignal => write!(f, "Interrupted by signal"),
            SyscallError::ProcessExited(code) => write!(f, "Process exited with code {}", code),
            SyscallError::NotInitialized => write!(f, "Syscall system not initialized"),
        }
    }
}

impl std::error::Error for SyscallError {}

// ============================================================================
// TESTING
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syscall_register() {
        let mut dispatcher = SyscallDispatcher::new();
        
        dispatcher.register(SYS_WRITE, SyscallHandler {
            name: "write",
            handler: |_| Ok(0),
        });
        
        assert!(dispatcher.handlers.contains_key(&SYS_WRITE));
    }
    
    #[test]
    fn test_syscall_dispatch() {
        let mut dispatcher = SyscallDispatcher::new();
        
        dispatcher.register(SYS_WRITE, SyscallHandler {
            name: "write",
            handler: |_| Ok(100),
        });
        
        dispatcher.start();
        
        let result = dispatcher.invoke(SYS_WRITE, &[]).unwrap();
        assert_eq!(result, 100);
    }
    
    #[test]
    fn test_getpid() {
        let mut dispatcher = SyscallDispatcher::new();
        
        dispatcher.register(SYS_GETPID, SyscallHandler {
            name: "getpid",
            handler: |_| Ok(1),
        });
        
        dispatcher.start();
        
        let pid = dispatcher.invoke(SYS_GETPID, &[]).unwrap();
        assert_eq!(pid, 1);
    }
}
