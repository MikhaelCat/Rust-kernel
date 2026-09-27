//! IPC (Inter-Process Communication) Mechanisms for Linux Kernel on Rust
//! Реализация механизмов межпроцессного взаимодействия

use std::collections::VecDeque;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;

// ============================================================================
// SHARED MEMORY MANAGMENT
// ============================================================================

#[derive(Debug, Clone)]
pub struct ShmSegment {
    pub id: u32,
    pub addr: *mut u8,
    pub size: usize,
    pub attached_count: u32,
    pub perms: PermissionFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShmCmd {
    ShmGetInfo,
    ShmSet,
    ShmRm,
}

#[derive(Debug)]
pub struct ShmInfo {
    pub shm_seg_num: u32,
    pub shm_tot: usize,
    pub shm_rss: usize,
    pub shm_swp: usize,
}

pub struct SharedMemory {
    segments: HashMap<u32, ShmSegment>,
    next_id: AtomicU32,
}

impl Default for SharedMemory {
    fn default() -> Self {
        Self {
            segments: HashMap::new(),
            next_id: AtomicU32::new(1),
        }
    }
}

impl SharedMemory {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Allocate shared memory segment
    pub fn shmget(&mut self, size: usize) -> Result<u32, ShmError> {
        if size == 0 || size > MAX_SHM_SIZE {
            return Err(ShmError::InvalidArgument);
        }
        
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        
        // In production, would allocate physically contiguous memory
        let addr = unsafe { libc::malloc(size as usize) as *mut u8 };
        
        if addr.is_null() {
            return Err(ShmError::OutOfMemory);
        }
        
        let segment = ShmSegment {
            id,
            addr,
            size,
            attached_count: 0,
            perms: PermissionFlags::default(),
        };
        
        self.segments.insert(id, segment);
        Ok(id)
    }
    
    /// Attach to shared memory segment
    pub fn shmat(&mut self, id: u32) -> Result<*mut u8, ShmError> {
        let segment = self.segments.get(&id).ok_or(ShmError::NotFound)?;
        
        segment.attached_count += 1;
        Ok(segment.addr)
    }
    
    /// Detach from shared memory
    pub fn shmdt(&mut self, id: u32) -> Result<(), ShmError> {
        let segment = self.segments.get_mut(&id).ok_or(ShmError::NotFound)?;
        
        if segment.attached_count > 0 {
            segment.attached_count -= 1;
        }
        
        Ok(())
    }
    
    /// Control operations
    pub fn shmctl(&mut self, id: u32, cmd: ShmCmd) -> Result<ShmInfo, ShmError> {
        match cmd {
            ShmCmd::ShmGetInfo => {
                let segment = self.segments.get(&id).ok_or(ShmError::NotFound)?;
                Ok(ShmInfo {
                    shm_seg_num: self.segments.len() as u32,
                    shm_tot: segment.size,
                    shm_rss: segment.size,
                    shm_swp: 0,
                })
            },
            ShmCmd::ShmSet => Ok(ShmInfo {
                shm_seg_num: self.segments.len() as u32,
                shm_tot: 0,
                shm_rss: 0,
                shm_swp: 0,
            }),
            ShmCmd::ShmRm => {
                if let Some(segment) = self.segments.remove(&id) {
                    unsafe { libc::free(segment.addr as *mut libc::c_void); }
                    Ok(ShmInfo {
                        shm_seg_num: self.segments.len() as u32,
                        shm_tot: 0,
                        shm_rss: 0,
                        shm_swp: 0,
                    })
                } else {
                    Err(ShmError::NotFound)
                }
            },
        }
    }
}

// ============================================================================
// POSIX SEMAPHORES
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemFlags {
    IPCKEY,
    SEM_ALLOC,
    SEM_ANON,
}

#[derive(Debug)]
pub struct SemaphoreContext {
    value: u32,
    max: u32,
    waiters: Vec<Arc<ConditionVariable>>,
}

impl SemaphoreContext {
    pub fn new(value: u32, max: u32) -> Self {
        Self {
            value,
            max,
            waiters: Vec::new(),
        }
    }
    
    pub fn try_wait(&mut self) -> Result<(), TryWaitError> {
        if self.value > 0 {
            self.value -= 1;
            Ok(())
        } else {
            Err(TryWaitError::WouldBlock)
        }
    }
    
    pub fn wait(&mut self) -> Result<(), SemError> {
        loop {
            if self.value > 0 {
                self.value -= 1;
                return Ok(());
            }
            
            // Wait for signal - simplified version
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    
    pub fn post(&mut self) -> Result<(), SemError> {
        if self.value < self.max {
            self.value += 1;
            Ok(())
        } else {
            Err(SemError::SemaphoreOverflow)
        }
    }
    
    pub fn get_value(&self) -> u32 {
        self.value
    }
}

// ============================================================================
// SYSTEM V SEMAPHORES
// ============================================================================

pub struct SysVSemaphoreManager {
    sets: HashMap<u32, SysVSemSet>,
    next_id: AtomicU32,
}

pub struct SysVSemSet {
    pub semid: u32,
    pub sems: Vec<SysVSemOp>,
    pub nsems: usize,
    pub perms: PermissionFlags,
    pub pid: Option<pid_t>,
    pub cpid: pid_t,
    pub use_ctime: bool,
    pub attchcnt: u16,
    pub seq: u16,
}

pub struct SysVSemOp {
    pub sem_num: u16,
    pub sem_op: i16,
    pub sem_flg: u16,
}

impl Default for SysVSemaphoreManager {
    fn default() -> Self {
        Self {
            sets: HashMap::new(),
            next_id: AtomicU32::new(1),
        }
    }
}

impl SysVSemaphoreManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn semget(&mut self, key: i32, nsem: u32, semflg: u32) -> Result<i32, SemError> {
        if nsem == 0 || nsem as usize > MAX_SEMS_PER_SET {
            return Err(SemError::InvalidArgument);
        }
        
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        
        let mut sems = Vec::with_capacity(nsem as usize);
        for _ in 0..nsem {
            sems.push(SysVSemOp {
                sem_num: 0,
                sem_op: 0,
                sem_flg: 0,
            });
        }
        
        let set = SysVSemSet {
            semid: id as i32,
            sems,
            nsems: nsem as usize,
            perms: PermissionFlags::default(),
            pid: None,
            cpid: unsafe { libc::getpid() },
            use_ctime: false,
            attchcnt: 0,
            seq: 0,
        };
        
        self.sets.insert(id, set);
        Ok(id as i32)
    }
}

// ============================================================================
// MESSAGE QUEUES
// ============================================================================

#[derive(Debug)]
pub struct MessageQueueContext {
    msgqnum: u32,
    msg_tqlen: i32,
    msg_nextid: u32,
    msgsndt: u64,
    msg_lrpid: pid_t,
    msg_alltrs: u64,
}

pub struct KernelMessage {
    pub mtype: u64,           // Must be > 0
    pub mtext: String,
    pub mpid: pid_t,
    pub ts: u64,
    pub qnum: u32,
}

pub struct MessageQueue {
    messages: VecDeque<KernelMessage>,
    max_messages: u32,
    max_msgsize: u32,
}

impl MessageQueue {
    pub fn new(max_messages: u32, max_msgsize: u32) -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages,
            max_msgsize,
        }
    }
    
    pub fn mq_send(&mut self, mtype: u64, text: &str) -> Result<(), MsgError> {
        if mtype == 0 || mtype > u64::MAX / 2 {
            return Err(MsgError::InvalidMessageType);
        }
        
        if text.len() > self.max_msgsize as usize {
            return Err(MsgError::MessageTooLarge);
        }
        
        if self.messages.len() >= self.max_messages as usize {
            return Err(MsgError::QueueFull);
        }
        
        let msg = KernelMessage {
            mtype,
            mtext: text.to_string(),
            mpid: unsafe { libc::getpid() },
            ts: get_timestamp_ns(),
            qnum: self.messages.len() as u32,
        };
        
        self.messages.push_back(msg);
        Ok(())
    }
    
    pub fn mq_receive(&mut self, type_filter: u64) -> Result<KernelMessage, MsgError> {
        let pos = self.messages.iter().position(|m| {
            type_filter == 0 || m.mtype == type_filter
        });
        
        match pos {
            Some(idx) => {
                let msg = self.messages.remove(idx).unwrap();
                Ok(msg)
            },
            None => Err(MsgError::MessageNotFound),
        }
    }
}

// ============================================================================
// SIGNALS AND NOTIFICATIONS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignalNumber {
    SIGHUP = 1,
    SIGINT = 2,
    SIGQUIT = 3,
    SIGILL = 4,
    SIGABRT = 6,
    SIGFPE = 8,
    SIGKILL = 9,
    SIGSEGV = 11,
    SIGPIPE = 13,
    SIGALRM = 14,
    SIGTERM = 15,
    // ... more signals supported
}

#[derive(Debug, Clone, Copy)]
pub enum SignalHandler {
    Default,
    Ignore,
    Handler(fn(SignalNumber)),
    Action(Box<dyn Fn(SignalNumber) + Send>),
}

#[derive(Debug, Clone)]
pub struct SaFlag {
    sa_restorer: Option<RestorerFunc>,
    sa_mask: SignalSet,
    sa_flags: u16,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct SignalSet(u64);

impl SignalSet {
    pub fn add(&mut self, sig: SignalNumber) {
        self.0 |= 1 << (sig as u64 % 64);
    }
    
    pub fn remove(&mut self, sig: SignalNumber) {
        self.0 &= !(1 << (sig as u64 % 64));
    }
    
    pub fn contains(&self, sig: SignalNumber) -> bool {
        self.0 & (1 << (sig as u64 % 64)) != 0
    }
}

#[derive(Default)]
pub struct SignalContext {
    pending: u64,
    blocked: SignalSet,
    caught: HashMap<SignalNumber, SaFlag>,
}

// ============================================================================
// PIPE SUPPORT
// ============================================================================

pub struct PipeBuffer {
    data: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
    can_read: Arc<AtomicBool>,
    can_write: Arc<AtomicBool>,
}

pub struct PipeEnds {
    pub read_fd: i32,
    pub write_fd: i32,
}

// ============================================================================
// ERRORS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ShmError {
    InvalidArgument,
    NotFound,
    AccessDenied,
    OutOfMemory,
    ResourceBusy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SemError {
    InvalidArgument,
    SemaphoreExists,
    NoMem,
    TooManySemaphores,
    SemaphoreInUse,
    InvalidId,
    OperationNotPermitted,
    BadAddress,
    SemaphoreOverflow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TryWaitError {
    WouldBlock,
    InvalidSemaphore,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MsgError {
    InvalidMessageType,
    MessageTooLarge,
    QueueFull,
    MessageNotFound,
    InvalidQueue,
}

// Helper functions
fn get_timestamp_ns() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

const MAX_SHM_SIZE: usize = 64 * 1024 * 1024;  // 64 MB
const MAX_SEMS_PER_SET: usize = 128;

type pid_t = i32;
type key_t = i32;

#[derive(Debug, Clone)]
pub struct PermissionFlags(u16);

impl Default for PermissionFlags {
    fn default() -> Self {
        Self(0o600) // User read/write
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_shared_memory_allocation() {
        let mut shm = SharedMemory::new();
        
        let id = shm.shmget(4096).unwrap();
        assert_eq!(id, 1);
        
        let ptr = shm.shmat(id).unwrap();
        assert!(!ptr.is_null());
        
        unsafe {
            *(ptr as *mut u32) = 100;
            assert_eq!(*(ptr as *const u32), 100);
        }
        
        shm.shmdt(id).unwrap();
        shm.shmctl(id, ShmCmd::ShmRm).unwrap();
    }
    
    #[test]
    fn test_semaphore_operations() {
        let mut sem = SemaphoreContext::new(1, 10);
        
        sem.wait().unwrap();
        assert_eq!(sem.get_value(), 0);
        
        sem.post().unwrap();
        assert_eq!(sem.get_value(), 1);
    }
    
    #[test]
    fn test_sysv_semaphore_creation() {
        let mut mgr = SysVSemaphoreManager::new();
        
        let id = mgr.semget(1234, 5, 0o600).unwrap();
        assert!(id > 0);
    }
    
    #[test]
    fn test_message_queue() {
        let mut mq = MessageQueue::new(10, 256);
        
        mq.mq_send(1, "Hello").unwrap();
        
        let msg = mq.mq_receive(1).unwrap();
        assert_eq!(msg.mtext, "Hello");
        assert_eq!(msg.mtype, 1);
    }
    
    #[test]
    fn test_signal_set_operations() {
        let mut mask = SignalSet::default();
        
        mask.add(SignalNumber::SIGTERM);
        assert!(mask.contains(SignalNumber::SIGTERM));
        
        mask.remove(SignalNumber::SIGTERM);
        assert!(!mask.contains(SignalNumber::SIGTERM));
    }
}

use std::collections::HashMap;
use libc;
