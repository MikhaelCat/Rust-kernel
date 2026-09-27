//! Real-Time (RT) Scheduler Implementation for Linux Kernel
//! 
//! Реализация планировщика реального времени с приоритетным управлением.
//! Поддерживает два алгоритма: SCHED_FIFO и SCHED_RR

use crate::sched::error::SchedError;
use crate::sched::types::Task;
use std::collections::VecDeque;

/// Приоритеты RT задач (1-99, где 99 - максимальный)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtPrio(pub u8);

impl RtPrio {
    pub const MIN: u8 = 1;
    pub const MAX: u8 = 99;
    pub const NORMAL: u8 = 0; //为非 RT задачи
    
    pub fn new(prio: u8) -> Result<Self, SchedError> {
        if prio == 0 || prio > Self::MAX {
            return Err(SchedError::InvalidPriority);
        }
        Ok(RtPrio(prio))
    }
    
    pub fn is_rt(&self) -> bool {
        self.0 > 0 && self.0 <= Self::MAX
    }
    
    pub fn as_u8(&self) -> u8 {
        self.0
    }
}

/// Тип планирования RT задач
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtPolicy {
    /// SCHED_FIFO: задача выполняется пока не закончится или не отдаст CPU
    Fifo,
    /// SCHED_RR: Round-Robin для задач с одинаковым приоритетом
    RoundRobin,
}

/// Таймаут кванта времени для RR
#[derive(Debug, Clone, Copy)]
pub struct RtTimeRange {
    pub rr_settime: u32, // Время кванта в миллисекундах
}

impl Default for RtTimeRange {
    fn default() -> Self {
        Self {
            rr_settime: 500, // 500ms стандартный квант
        }
    }
}

/// Run Queue для RT задач на конкретном CPU
#[derive(Debug)]
pub struct RtRunQueue {
    pub cpu_id: usize,
    pub policy: RtPolicy,
    pub time_range: RtTimeRange,
    pub priority_queues: [Option<VecDeque<RtTask>>; 100], // 100 уровней приоритетов
    pub current_priority: RtPrio,
    pub active_tasks: usize,
}

impl Default for RtRunQueue {
    fn default() -> Self {
        Self::new(0)
    }
}

impl RtRunQueue {
    pub fn new(cpu_id: usize) -> Self {
        Self {
            cpu_id,
            policy: RtPolicy::Fifo,
            time_range: RtTimeRange::default(),
            priority_queues: Default::default(),
            current_priority: RtPrio::new(RtPrio::MIN).unwrap(),
            active_tasks: 0,
        }
    }
    
    /// Добавить задачу в очередь по приоритету
    pub fn enqueue_task(&mut self, task: &mut Task) -> Result<(), SchedError> {
        let prio = RtPrio::new(task.rt_prio.as_u8())?;
        
        if self.priority_queues[prio.as_u8() as usize].is_none() {
            self.priority_queues[prio.as_u8() as usize] = Some(VecDeque::new());
        }
        
        let queue = self.priority_queues[prio.as_u8() as usize].as_mut().unwrap();
        queue.push_back(RtTask::from_task(task));
        self.active_tasks += 1;
        
        // Обновляем текущий приоритет если это выше
        if prio.as_u8() > self.current_priority.as_u8() {
            self.current_priority = prio;
        }
        
        Ok(())
    }
    
    /// Удалить задачу из очереди
    pub fn dequeue_task(&mut self, pid: u32) -> Result<Option<RtTask>, SchedError> {
        // Ищем задачу по всем уровням приоритетов
        for priority in self.current_priority.as_u8()..=RtPrio::MAX {
            if let Some(queue) = &mut self.priority_queues[priority as usize] {
                if let Some(pos) = queue.iter().position(|t| t.pid == pid) {
                    let task = queue.remove(pos).unwrap();
                    self.active_tasks -= 1;
                    return Ok(Some(task));
                }
            }
        }
        Err(SchedError::NoSuchProcess)
    }
    
    /// Выбрать следующую задачу для выполнения (с высшим приоритетом)
    pub fn pick_next_task(&mut self) -> Option<&mut RtTask> {
        // Находим самый высокий непустой приоритет
        for priority in self.current_priority.as_u8()..=RtPrio::MAX {
            if let Some(queue) = &mut self.priority_queues[priority as usize] {
                if !queue.is_empty() {
                    // Для RR возвращаем первого, для FIFO тоже первого
                    return queue.get_mut(0);
                }
            }
        }
        None
    }
    
    /// Получить следующую задачу без удаления
    pub fn peek_next_task(&self) -> Option<&RtTask> {
        for priority in self.current_priority.as_u8()..=RtPrio::MAX {
            if let Some(queue) = &self.priority_queues[priority as usize] {
                if !queue.is_empty() {
                    return queue.front();
                }
            }
        }
        None
    }
    
    /// Квантование времени для RR (когда у задачи закончился квант)
    pub fn expire_quantum(&mut self, pid: u32) -> Result<(), SchedError> {
        match self.policy {
            RtPolicy::Fifo => {
                // Для FIFO нет квантования
                Ok(())
            }
            RtPolicy::RoundRobin => {
                // Возвращаем задачу в конец очереди ее приоритета
                if let Some(queue_opt) = self.priority_queues[self.current_priority.as_u8() as usize].as_mut() {
                    if let Some(pos) = queue_opt.iter().position(|t| t.pid == pid) {
                        let task = queue_opt.remove(pos).unwrap();
                        queue_opt.push_back(task);
                        Ok(())
                    } else {
                        Err(SchedError::NoSuchProcess)
                    }
                } else {
                    Err(SchedError::NoSuchProcess)
                }
            }
        }
    }
    
    /// Количество активных задач
    pub fn count_tasks(&self) -> usize {
        self.active_tasks
    }
    
    /// Переключение политики планирования
    pub fn set_policy(&mut self, policy: RtPolicy) {
        self.policy = policy;
    }
    
    /// Установка времени кванта
    pub fn set_time_range(&mut self, time_range: RtTimeRange) {
        self.time_range = time_range;
    }
}

/// Структура задачи для RT планировщика
#[derive(Debug, Clone)]
pub struct RtTask {
    pub pid: u32,
    pub name: String,
    pub prio: RtPrio,
    pub exec_time: u64,      // Estimated execution time
    pub period: u64,         // Period for periodic tasks
    pub deadline: u64,       // Absolute deadline
    pub runtime: u32,        // Remaining runtime in quantum
    pub flags: RtTaskFlags,
}

impl RtTask {
    pub fn from_task(task: &Task) -> Self {
        Self {
            pid: task.pid,
            name: task.comm.clone(),
            prio: task.rt_prio,
            exec_time: 10000, // Default estimate
            period: 0,
            deadline: 0,
            runtime: 5, // Default quantum
            flags: RtTaskFlags::empty(),
        }
    }
    
    /// Проверка есть ли время на выполнение
    pub fn has_runtime(&self) -> bool {
        self.runtime > 0
    }
    
    /// Уменьшить оставшееся время
    pub fn consume_runtime(&mut self, amount: u32) {
        if self.runtime >= amount {
            self.runtime -= amount;
        } else {
            self.runtime = 0;
        }
    }
    
    /// Проверка истек ли дедлайн
    pub fn check_deadline(&self, current_time: u64) -> bool {
        current_time >= self.deadline
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RtTaskFlags(u32);

impl RtTaskFlags {
    pub const NONE: RtTaskFlags = RtTaskFlags(0);
    pub const SCHED_FLAG_RESET_ON_FORK: RtTaskFlags = RtTaskFlags(1 << 0);
    pub const SCHED_FLAG_REUSE_ID: RtTaskFlags = RtTaskFlags(1 << 1);
    pub const SCHED_FLAG_ALL: RtTaskFlags = RtTaskFlags(3);
    
    pub fn empty() -> Self {
        Self::NONE
    }
    
    pub fn all() -> Self {
        Self::SCHED_FLAG_ALL
    }
}

/// Статистика RT планировщика
#[derive(Debug, Clone)]
pub struct RtStats {
    pub nr_schedule_cycles: u64,
    pub nr_switches: u64,
    pub nr_preemptions: u64,
    pub nr_context_switches: u64,
    pub highprio_tasks_run: u64,
}

impl Default for RtStats {
    fn default() -> Self {
        Self {
            nr_schedule_cycles: 0,
            nr_switches: 0,
            nr_preemptions: 0,
            nr_context_switches: 0,
            highprio_tasks_run: 0,
        }
    }
}

impl RtStats {
    pub fn tick(&mut self) {
        self.nr_schedule_cycles += 1;
    }
    
    pub fn on_switch(&mut self) {
        self.nr_switches += 1;
    }
    
    pub fn on_preemption(&mut self) {
        self.nr_preemptions += 1;
    }
}

/// Менеджер RT планирования
pub struct RtScheduler {
    pub runqueues: Vec<RtRunQueue>,
    pub global_stats: RtStats,
    pub enabled: bool,
}

impl Default for RtScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl RtScheduler {
    pub fn new() -> Self {
        Self {
            runqueues: vec![RtRunQueue::new(0)],
            global_stats: RtStats::default(),
            enabled: true,
        }
    }
    
    /// Создать новый RT ранкью для CPU
    pub fn add_cpu(&mut self, cpu_id: usize) {
        self.runtimes.push(RtRunQueue::new(cpu_id));
    }
    
    /// Удалить RT ранкью
    pub fn remove_cpu(&mut self, cpu_id: usize) {
        if let Some(pos) = self.runtimes.iter().position(|r| r.cpu_id == cpu_id) {
            self.runtimes.remove(pos);
        }
    }
    
    /// Запустить задачу на конкретном CPU
    pub fn rt_enqueue_task(&mut self, cpu_id: usize, task: &mut Task) -> Result<(), SchedError> {
        if let Some(rq) = self.runtimes.get_mut(cpu_id) {
            rq.enqueue_task(task)
        } else {
            Err(SchedError::InvalidCpuId)
        }
    }
    
    /// Выбрать следующую RT задачу
    pub fn rt_pick_next_task(&mut self, cpu_id: usize) -> Option<RtTask> {
        self.runtimes.get_mut(cpu_id)?.pick_next_task().map(|t| t.clone())
    }
    
    /// Деку задачу с ранкью
    pub fn rt_dequeue_task(&mut self, cpu_id: usize, pid: u32) -> Result<Option<RtTask>, SchedError> {
        if let Some(rq) = self.runtimes.get_mut(cpu_id) {
            rq.dequeue_task(pid)
        } else {
            Err(SchedError::InvalidCpuId)
        }
    }
    
    /// Обработка тика таймера для RT
    pub fn rt_tick(&mut self, cpu_id: usize, current_pid: u32) -> Option<u32> {
        self.global_stats.tick();
        
        if let Some(rq) = self.runtimes.get_mut(cpu_id) {
            if let Some(current) = rq.pick_next_task() {
                // Уменьшаем runtime
                current.consume_runtime(1);
                
                // Проверяем истек ли квант
                if !current.has_runtime() {
                    rq.expire_quantum(current_pid).ok();
                }
                
                return Some(current.pid);
            }
        }
        None
    }
    
    /// Получение количества CPU
    pub fn cpu_count(&self) -> usize {
        self.runtimes.len()
    }
    
    /// Включить/выключить RT планировщик
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Проверка есть ли RT задачи
    pub fn has_rt_tasks(&self, cpu_id: usize) -> bool {
        self.runtimes.get(cpu_id)
            .map(|rq| rq.count_tasks() > 0)
            .unwrap_or(false)
    }
}
