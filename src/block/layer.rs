//! Block Layer I/O Implementation for Linux Kernel on Rust
//! Реализация слоя блочных устройств и планировщиков I/O

use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::Mutex;
use atomic_waker::AtomicWaker;

/// Типы операций I/O
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOMOperation {
    READ,          // Чтение с устройства
    WRITE,         // Запись на устройство
    FLUSH,         // Принудительная запись кэша
    SYNCHRONIZE,   // Синхронизация кэша
    DISCARD,       // TRIM/DISCARD для SSD
}

/// Блок устройства
#[derive(Debug)]
pub struct BlockDevice {
    pub name: String,
    pub dev_id: u32,
    pub size: usize,                    // Размер в байтах
    pub sector_size: usize,             // Размер сектора (512/4096)
    pub queue_depth: usize,             // Глубина очереди
    pub read_only: bool,
    pub media_type: MediaType,
    pub stats: DeviceStats,
    pub queue: RequestQueue,
}

impl BlockDevice {
    pub fn new(name: &str, dev_id: u32, size_bytes: usize, sector_size: usize) -> Self {
        Self {
            name: name.to_string(),
            dev_id,
            size: size_bytes,
            sector_size,
            queue_depth: 128,
            read_only: false,
            media_type: MediaType::HDD,
            stats: DeviceStats::new(),
            queue: RequestQueue::new(128),
        }
    }
    
    /// Получить количество секторов
    pub fn num_sectors(&self) -> u64 {
        (self.size / self.sector_size) as u64
    }
    
    /// Проверить легитимность области доступа
    pub fn is_valid_range(&self, start_sector: u64, num_sectors: u64) -> bool {
        if start_sector + num_sectors > self.num_sectors() {
            return false;
        }
        
        // Выравнивание по границе сектора
        start_sector as usize % self.sector_size == 0 &&
        num_sectors as usize % self.sector_size == 0
    }
    
    /// Подать запрос на выполнение
    pub fn submit_request(&mut self, request: BlockRequest) -> Result<(), IO_Error> {
        if self.read_only && matches!(request.operation, IOMOperation::WRITE | 
                                       IOMOperation::DISCARD |
                                       IOMOperation::FLUSH)
        {
            return Err(IO_Error::ReadOnlyFilesystem);
        }
        
        self.queue.request(request)
    }
}

/// Тип носителя данных
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MediaType {
    HDD,              // Жесткий диск
    SSD,              // Твердотельный накопитель
    CDROM,            // Оптический диск
    Flash,            // USB/SD карта
    RAM,              // RAM диск
    Virtual,          // Виртуальное устройство
}

/// Статистика устройства
#[derive(Debug)]
pub struct DeviceStats {
    pub reads_total: AtomicU64,
    pub reads_merged: AtomicU64,
    pub sectors_read: AtomicU64,
    pub read_time_ns: AtomicU64,
    
    pub writes_total: AtomicU64,
    pub writes_merged: AtomicU64,
    pub sectors_written: AtomicU64,
    pub write_time_ns: AtomicU64,
    
    pub io_in_progress: AtomicU32,
    pub ios_completed: AtomicU64,
    pub last_update: AtomicU64,
}

impl DeviceStats {
    pub fn new() -> Self {
        Self {
            reads_total: AtomicU64::new(0),
            reads_merged: AtomicU64::new(0),
            sectors_read: AtomicU64::new(0),
            read_time_ns: AtomicU64::new(0),
            writes_total: AtomicU64::new(0),
            writes_merged: AtomicU64::new(0),
            sectors_written: AtomicU64::new(0),
            write_time_ns: AtomicU64::new(0),
            io_in_progress: AtomicU32::new(0),
            ios_completed: AtomicU64::new(0),
            last_update: AtomicU64::new(0),
        }
    }
    
    pub fn record_read(&self, sectors: usize) {
        self.reads_total.fetch_add(1, Ordering::Relaxed);
        self.sectors_read.fetch_add(sectors as u64, Ordering::Relaxed);
    }
    
    pub fn record_write(&self, sectors: usize) {
        self.writes_total.fetch_add(1, Ordering::Relaxed);
        self.sectors_written.fetch_add(sectors as u64, Ordering::Relaxed);
    }
}

/// Флаги очереди
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueFlags(u32);

impl QueueFlags {
    pub const NONE: u32 = 0;
    pub const FIFO: u32 = 1 << 0;           // FIFO очередь
    pub const SYNC: u32 = 1 << 1;           // синхронные операции
    pub const QUEUE_STOPPED: u32 = 1 << 2;  // Очередь приостановлена
    pub const DISCARD: u32 = 1 << 3;        // поддерживает TRIM
}

/// Очередь запросов
#[derive(Debug)]
pub struct RequestQueue {
    pub depth: usize,                       // Максимальная глубина
    pub pending_requests: VecDeque<BlockRequest>,
    pub dispatch_list: Vec<BlockRequest>,
    pub flags: QueueFlags,
    pub stats: QueueStats,
    scheduler: Box<dyn IScheduler>,
}

impl RequestQueue {
    pub fn new(depth: usize) -> Self {
        Self {
            depth,
            pending_requests: VecDeque::new(),
            dispatch_list: Vec::new(),
            flags: QueueFlags::NONE,
            stats: QueueStats::new(),
            scheduler: Box::<NoopScheduler>::default(),
        }
    }
    
    /// Добавить запрос в очередь
    pub fn request(&mut self, req: BlockRequest) -> Result<(), IO_Error> {
        if self.pending_requests.len() >= self.depth {
            return Err(IO_Error::QueueFull);
        }
        
        // Попытка слияния с соседними запросами
        if self.can_merge(&req) {
            self.merge_request(req);
            self.stats.merge_count += 1;
            return Ok(());
        }
        
        self.pending_requests.push_back(req);
        Ok(())
    }
    
    /// Проверить возможность слияния
    fn can_merge(&self, req: &BlockRequest) -> bool {
        if let Some(last) = self.pending_requests.back() {
            if last.dev_id == req.dev_id && 
               last.operation == req.operation &&
               last.sector + last.sectors == req.sector
            {
                return true;
            }
        }
        false
    }
    
    /// Слить запрос с существующим
    fn merge_request(&mut self, mut new_req: BlockRequest) {
        if let Some(last) = self.pending_requests.back_mut() {
            last.sectors += new_req.sectors;
        }
    }
    
    /// Получить следующий запрос из планировщика
    pub fn next_request(&mut self) -> Option<&mut BlockRequest> {
        if self.dispatch_list.is_empty() {
            // Использовать планировщик для выбора следующего
            if let Some(next) = self.scheduler.select_next() {
                self.dispatch_list.push(next);
            }
        }
        
        self.dispatch_list.pop_front()
    }
    
    /// Завершить запрос
    pub fn complete(&mut self, id: u64) -> Result<(), IO_Error> {
        if let Some(pos) = self.dispatch_list.iter().position(|r| r.request_id == id) {
            let req = self.dispatch_list.remove(pos);
            self.stats.complete_count += 1;
            req.completion.signal(0)?;
            Ok(())
        } else {
            Err(IO_Error::InvalidRequest)
        }
    }
    
    /// Установить планировщик
    pub fn set_scheduler(&mut self, scheduler: Box<dyn IScheduler>) {
        self.scheduler = scheduler;
    }
}

/// Запрос блочного I/O
#[derive(Debug)]
pub struct BlockRequest {
    pub request_id: u64,
    pub dev_id: u32,
    pub operation: IOMOperation,
    pub sector: u64,                      // Начальный сектор
    pub sectors: usize,                   // Количество секторов
    pub data: Option<Vec<u8>>,            // Данные для записи
    pub completion: Arc<Completion>,      // Callback завершения
}

impl BlockRequest {
    pub fn new(dev_id: u32, operation: IOMOperation, sector: u64, sectors: usize) -> Self {
        Self {
            request_id: generate_request_id(),
            dev_id,
            operation,
            sector,
            sectors,
            data: None,
            completion: Arc::new(Completion::new()),
        }
    }
    
    pub fn read(dev_id: u32, sector: u64, sectors: usize) -> Self {
        Self::new(dev_id, IOMOperation::READ, sector, sectors)
    }
    
    pub fn write(dev_id: u32, sector: u64, sectors: usize, data: Vec<u8>) -> Self {
        assert_eq!(data.len(), sectors * 512); // Предполагаем 512-байтовые сектора
        
        Self {
            request_id: generate_request_id(),
            dev_id,
            operation: IOMOperation::WRITE,
            sector,
            sectors,
            data: Some(data),
            completion: Arc::new(Completion::new()),
        }
    }
    
    /// Получить размер в байтах
    pub fn size(&self) -> usize {
        self.sectors * 512
    }
}

fn generate_request_id() -> u64 {
    static mut COUNTER: u64 = 0;
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

/// Уведомление о завершении
pub struct Completion {
    done: Mutex<bool>,
    error: Mutex<Option<i32>>,
}

impl Completion {
    pub fn new() -> Self {
        Self {
            done: Mutex::new(false),
            error: Mutex::new(None),
        }
    }
    
    pub fn wait(&self) -> Result<(), IO_Error> {
        while !*self.done.lock() {
            // Yield to other tasks
            std::thread::yield_now();
        }
        
        let err = *self.error.lock();
        match err {
            Some(e) if e != 0 => Err(IO_Error::IoError(e)),
            _ => Ok(()),
        }
    }
    
    pub fn signal(&self, error_code: i32) {
        let mut done = self.done.lock();
        let mut err = self.error.lock();
        
        *done = true;
        *err = if error_code == 0 { None } else { Some(error_code) };
    }
}

/// Планировщик I/O - трейт
pub trait IScheduler: Send {
    fn select_next(&mut self) -> Option<BlockRequest>;
    fn insert(&mut self, req: BlockRequest);
    fn expired_requests(&self) -> Vec<BlockRequest>;
}

// ============================================================================
// NOOP SCHEDULER (FIFO)
// ============================================================================

#[derive(Debug)]
pub struct NoopScheduler {
    queue: VecDeque<BlockRequest>,
}

impl Default for NoopScheduler {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}

impl IScheduler for NoopScheduler {
    fn select_next(&mut self) -> Option<BlockRequest> {
        self.queue.pop_front()
    }
    
    fn insert(&mut self, req: BlockRequest) {
        self.queue.push_back(req);
    }
    
    fn expired_requests(&self) -> Vec<BlockRequest> {
        Vec::new() // Noop doesn't track deadlines
    }
}

// ============================================================================
// DEADLINE SCHEDULER
// ============================================================================

#[derive(Debug)]
pub struct DeadlineScheduler {
    read_queue: VecDeque<BlockRequest>,
    write_queue: VecDeque<BlockRequest>,
    max_latency_ns: u64,
    starve_time_ns: u64,
}

impl Default for DeadlineScheduler {
    fn default() -> Self {
        Self {
            read_queue: VecDeque::new(),
            write_queue: VecDeque::new(),
            max_latency_ns: 128_000_000,     // 128ms в наносекундах
            starve_time_ns: 500_000_000,     // 500ms
        }
    }
}

impl DeadlineScheduler {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn set_max_latency(&mut self, latency_ns: u64) {
        self.max_latency_ns = latency_ns;
    }
    
    /// Проверить истекшие запросы
    fn check_expired(&mut self) -> Option<BlockRequest> {
        // Проверить читательские запросы
        for i in 0..self.read_queue.len() {
            // Здесь нужно хранить timestamp, но для упрощения просто берём первый
            if i > 0 {
                return Some(self.read_queue[i..].pop_front().unwrap());
            }
        }
        
        // Проверить записывающие запросы
        for i in 0..self.write_queue.len() {
            if i > 0 {
                return Some(self.write_queue[i..].pop_front().unwrap());
            }
        }
        
        None
    }
}

impl IScheduler for DeadlineScheduler {
    fn select_next(&mut self) -> Option<BlockRequest> {
        // Предпочитать читающие запросы
        if let Some(req) = self.check_expired() {
            return Some(req);
        }
        
        // Если нет истекших, брать из очереди читателей
        if let Some(req) = self.read_queue.front() {
            return Some(self.read_queue.pop_front().unwrap());
        }
        
        // Иначе брать запись
        self.write_queue.pop_front()
    }
    
    fn insert(&mut self, req: BlockRequest) {
        match req.operation {
            IOMOperation::READ | IOMOperation::SYNCHRONIZE => {
                self.read_queue.push_back(req);
            },
            _ => {
                self.write_queue.push_back(req);
            },
        }
    }
    
    fn expired_requests(&self) -> Vec<BlockRequest> {
        let mut expired = Vec::new();
        expired.extend(self.read_queue.clone());
        expired.extend(self.write_queue.clone());
        expired
    }
}

// ============================================================================
// CFQ SCHEDULER (Completely Fair Queuing)
// ============================================================================

#[derive(Debug)]
pub struct CfqScheduler {
    queues: Vec<VecDeque<BlockRequest>>,
    current_queue: usize,
    quantum: usize,
    slice_counter: usize,
}

impl Default for CfqScheduler {
    fn default() -> Self {
        Self {
            queues: vec![VecDeque::new()],
            current_queue: 0,
            quantum: 8,
            slice_counter: 0,
        }
    }
}

impl CfqScheduler {
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Добавить запрос от PID
    pub fn insert_for_pid(&mut self, pid: u32, req: BlockRequest) {
        let hash = pid as usize % self.queues.len();
        self.queues[hash].push_back(req);
    }
}

impl IScheduler for CfqScheduler {
    fn select_next(&mut self) -> Option<BlockRequest> {
        if self.slice_counter >= self.quantum {
            self.slice_counter = 0;
            self.current_queue = (self.current_queue + 1) % self.queues.len();
        }
        
        let queue = &mut self.queues[self.current_queue];
        if !queue.is_empty() {
            self.slice_counter += 1;
            queue.pop_front()
        } else {
            // Если текущая пуста, проверить другие
            for i in 0..self.queues.len() {
                let idx = (self.current_queue + i) % self.queues.len();
                if !self.queues[idx].is_empty() {
                    self.current_queue = idx;
                    return self.queues[idx].pop_front();
                }
            }
            None
        }
    }
    
    fn insert(&mut self, req: BlockRequest) {
        // CFQ использует queues by PID в реальной реализации
        // Здесь упрощено
        if !self.queues.is_empty() {
            self.queues[0].push_back(req);
        } else {
            self.queues.push(vec![req].into());
        }
    }
    
    fn expired_requests(&self) -> Vec<BlockRequest> {
        let mut expired = Vec::new();
        for queue in &self.queues {
            expired.extend(queue.clone());
        }
        expired
    }
}

// ============================================================================
// STATISTICS
// ============================================================================

#[derive(Debug)]
pub struct QueueStats {
    pub dispatch_count: u64,
    pub complete_count: u64,
    pub merge_count: u64,
    pub drop_count: u64,
    pub worst_latency_ns: u64,
    pub avg_wait_time_ns: AtomicU64,
}

impl QueueStats {
    pub fn new() -> Self {
        Self {
            dispatch_count: 0,
            complete_count: 0,
            merge_count: 0,
            drop_count: 0,
            worst_latency_ns: 0,
            avg_wait_time_ns: AtomicU64::new(0),
        }
    }
}

// ============================================================================
// ERRORS
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum IO_Error {
    InvalidRequest,
    QueueFull,
    ReadOnlyFilesystem,
    IoError(i32),
    Timeout,
}

impl std::fmt::Display for IO_Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IO_Error::InvalidRequest => write!(f, "Invalid request"),
            IO_Error::QueueFull => write!(f, "Request queue full"),
            IO_Error::ReadOnlyFilesystem => write!(f, "Read-only filesystem"),
            IO_Error::IoError(code) => write!(f, "IO error {}", code),
            IO_Error::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl std::error::Error for IO_Error {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_block_device_creation() {
        let device = BlockDevice::new("sda", 0, 1024 * 1024 * 1024, 512);
        
        assert_eq!(device.name, "sda");
        assert_eq!(device.dev_id, 0);
        assert_eq!(device.sector_size, 512);
        assert_eq!(device.num_sectors(), 2_097_152);
    }
    
    #[test]
    fn test_request_creation() {
        let req = BlockRequest::read(0, 100, 8);
        
        assert_eq!(req.dev_id, 0);
        assert_eq!(req.sector, 100);
        assert_eq!(req.sectors, 8);
        assert!(matches!(req.operation, IOMOperation::READ));
    }
    
    #[test]
    fn test_noop_scheduler() {
        let mut scheduler = NoopScheduler::default();
        
        let req1 = BlockRequest::read(0, 100, 8);
        let req2 = BlockRequest::read(0, 200, 8);
        
        scheduler.insert(req1);
        scheduler.insert(req2);
        
        assert!(scheduler.select_next().is_some());
        assert!(scheduler.select_next().is_some());
        assert!(scheduler.select_next().is_none());
    }
    
    #[test]
    fn test_deadline_scheduler() {
        let mut scheduler = DeadlineScheduler::default();
        
        let read_req = BlockRequest::read(0, 100, 8);
        let write_req = BlockRequest::write(0, 200, 8, vec![0u8; 4096]);
        
        scheduler.insert(read_req);
        scheduler.insert(write_req);
        
        // Should prefer reads over writes
        let selected = scheduler.select_next().unwrap();
        assert!(matches!(selected.operation, IOMOperation::READ));
    }
    
    #[test]
    fn test_cfq_scheduler() {
        let mut scheduler = CfqScheduler::default();
        
        scheduler.insert(BlockRequest::read(0, 100, 8));
        scheduler.insert(BlockRequest::read(0, 200, 8));
        
        assert!(scheduler.select_next().is_some());
        assert!(scheduler.select_next().is_some());
        assert!(scheduler.select_next().is_none());
    }
    
    #[test]
    fn test_completion_handling() {
        let completion = Arc::new(Completion::new());
        
        // Wait should block until signaled
        let completion_clone = Arc::clone(&completion);
        let handle = std::thread::spawn(move || {
            completion_clone.wait()
        });
        
        // Signal after short delay
        std::thread::sleep(std::time::Duration::from_millis(10));
        completion.signal(0);
        
        let result = handle.join().unwrap();
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_io_error_display() {
        assert_eq!(format!("{}", IO_Error::QueueFull), "Request queue full");
        assert_eq!(format!("{}", IO_Error::ReadOnlyFilesystem), "Read-only filesystem");
        assert_eq!(format!("{}", IO_Error::IoError(5)), "IO error 5");
    }
}
