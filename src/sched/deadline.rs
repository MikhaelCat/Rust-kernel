//! Deadine Scheduler Implementation for Linux Kernel
//! 
//! Планировщик задач с жесткими дедлайнами (SCHED_DEADLINE)
//! Использует алгоритм Global Earliest Deadline First (G-EDF)

use crate::sched::error::SchedError;
use crate::sched::types::Task;
use std::collections::BTreeMap;

/// Параметры задачи SCHED_DEADLINE
#[derive(Debug, Clone)]
pub struct DlParams {
    pub runtime: u64,        // Выделенное время выполнения в наносекундах
    pub period: u64,         // Период задачи в наносекундах
    pub deadline: u64,       // Дедлайн (обычно равен периоду)
}

impl Default for DlParams {
    fn default() -> Self {
        Self {
            runtime: 950_000_000,   // 950ms
            period: 1_000_000_000,  // 1s
            deadline: 1_000_000_000, // 1s
        }
    }
}

impl DlParams {
    /// Проверка валидности параметров
    pub fn validate(&self) -> Result<(), SchedError> {
        if self.runtime == 0 || self.period == 0 {
            return Err(SchedError::InvalidDeadlineParams);
        }
        if self.runtime > self.period {
            return Err(SchedError::InvalidDeadlineParams);
        }
        if self.deadline == 0 || self.deadline > self.period {
            return Err(SchedError::InvalidDeadlineParams);
        }
        Ok(())
    }
    
    /// Утилизация задачи (runtime/period)
    pub fn utilization(&self) -> f64 {
        self.runtime as f64 / self.period as f64
    }
    
    /// Сброс параметров по умолчанию
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Задача с дедлайном
#[derive(Debug, Clone)]
pub struct DlTask {
    pub pid: u32,
    pub name: String,
    pub params: DlParams,
    pub deadline_ptr: u64,    // Абсолютный дедлайн
    pub last_runtime: u64,    // Последнее использованное время
    curr_runtime: u64,        // Оставшееся время
    flags: DlTaskFlags,
}

impl DlTask {
    pub fn from_task(task: &Task, params: DlParams) -> Result<Self, SchedError> {
        params.validate()?;
        
        Ok(Self {
            pid: task.pid,
            name: task.comm.clone(),
            params: params.clone(), // Clone to avoid move
            deadline_ptr: 0,
            last_runtime: 0,
            curr_runtime: params.runtime,
            flags: DlTaskFlags::empty(),
        })
    }
    
    /// Обновить дедлайн для следующей активности
    pub fn update_deadline(&mut self, current_time: u64) {
        // Следующий дедлайн = текущий + период
        let next_deadline = (current_time / self.params.period + 1) * self.params.period;
        self.deadline_ptr = next_deadline;
        self.curr_runtime = self.params.runtime;
    }
    
    /// Проверяет истек ли дедлайн
    pub fn is_deadline_expired(&self, current_time: u64) -> bool {
        current_time >= self.deadline_ptr
    }
    
    /// Уменьшить оставшееся время выполнения
    pub fn consume_runtime(&mut self, amount: u64) {
        if self.curr_runtime >= amount {
            self.curr_runtime -= amount;
        } else {
            self.curr_runtime = 0;
        }
    }
    
    /// Есть ли еще время на выполнение
    pub fn has_runtime(&self) -> bool {
        self.curr_runtime > 0
    }
    
    /// Получить утилизацию
    pub fn utilization(&self) -> f64 {
        self.params.utilization()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DlTaskFlags(u32);

impl DlTaskFlags {
    pub const NONE: DlTaskFlags = DlTaskFlags(0);
    pub const SCHED_FLAG_RESET_ON_FORK: DlTaskFlags = DlTaskFlags(1 << 0);
    pub const SCHED_FLAG_REUSE_ID: DlTaskFlags = DlTaskFlags(1 << 1);
    pub const SCHED_FLAG_ALL: DlTaskFlags = DlTaskFlags(3);
    
    pub fn empty() -> Self { Self::NONE }
    pub fn all() -> Self { Self::SCHED_FLAG_ALL }
}

/// Очередь задач по дедлайну (BTreeMap сортирует по ключу)
#[derive(Debug)]
pub struct DeadlineRunQueue {
    pub cpu_id: usize,
    pub tasks: BTreeMap<u64, DlTask>, // Key = deadline_ptr, Value = DlTask
    pub total_utilization: f64,
    pub max_utilization: f64,         // Liu & Layland bound: n*(2^(1/n)-1)
    pub active_tasks: usize,
}

impl Default for DeadlineRunQueue {
    fn default() -> Self {
        Self::new(0)
    }
}

impl DeadlineRunQueue {
    pub fn new(cpu_id: usize) -> Self {
        Self {
            cpu_id,
            tasks: BTreeMap::new(),
            total_utilization: 0.0,
            max_utilization: 1.0, // 100% для Rate Monotonic
            active_tasks: 0,
        }
    }
    
    /// Добавить задачу по earliest deadline first
    pub fn enqueue_task(&mut self, task: DlTask) -> Result<(), SchedError> {
        // Проверка утилизации
        let util = task.utilization();
        if self.total_utilization + util > self.max_utilization {
            return Err(SchedError::DeadlineExceeded);
        }
        
        self.tasks.insert(task.deadline_ptr, task);
        self.total_utilization += util;
        self.active_tasks += 1;
        Ok(())
    }
    
    /// Удалить задачу
    pub fn dequeue_task(&mut self, pid: u32) -> Result<Option<DlTask>, SchedError> {
        let mut deleted = None;
        
        // Ищем задачу по PID
        let deadline_to_remove = self.tasks.iter()
            .find(|(_, task)| task.pid == pid)
            .map(|(dl, _)| *dl);
        
        if let Some(deadline) = deadline_to_remove {
            if let Some(task) = self.tasks.remove(&deadline) {
                self.total_utilization -= task.utilization();
                self.active_tasks -= 1;
                deleted = Some(task);
            }
        }
        
        if deleted.is_some() {
            Ok(deleted)
        } else {
            Err(SchedError::NoSuchProcess)
        }
    }
    
    /// Выбрать следующую задачу (с самым ранним дедлайном)
    pub fn pick_next_task(&mut self) -> Option<&mut DlTask> {
        // BTreeMap отсортирован по ключам, первый элемент имеет самый ранний дедлайн
        self.tasks.values_mut().next()
    }
    
    /// Следующий дедлайн
    pub fn next_deadline(&self) -> Option<u64> {
        self.tasks.keys().next().copied()
    }
    
    /// Количество активных задач
    pub fn count_tasks(&self) -> usize {
        self.active_tasks
    }
    
    /// Текущая утилизация
    pub fn utilization(&self) -> f64 {
        self.total_utilization
    }
    
    /// Проверка можно ли добавить новую задачу
    pub fn can_accept_task(&self, util: f64) -> bool {
        self.total_utilization + util <= self.max_utilization
    }
}

/// Статистика Deadline планировщика
#[derive(Debug, Clone)]
pub struct DlStats {
    pub nr_schedule_events: u64,
    pub nr_missed_deadlines: u64,
    pub nr_periodic_wakeup: u64,
    pub avg_waiting_time: u64,
    pub max_utilization_used: f64,
}

impl Default for DlStats {
    fn default() -> Self {
        Self {
            nr_schedule_events: 0,
            nr_missed_deadlines: 0,
            nr_periodic_wakeup: 0,
            avg_waiting_time: 0,
            max_utilization_used: 0.0,
        }
    }
}

impl DlStats {
    pub fn tick(&mut self) {
        self.nr_schedule_events += 1;
    }
    
    pub fn on_missed_deadline(&mut self) {
        self.nr_missed_deadlines += 1;
    }
    
    pub fn record_max_util(&mut self, util: f64) {
        if util > self.max_utilization_used {
            self.max_utilization_used = util;
        }
    }
}

/// Менеджер Deadline планирования
pub struct DlScheduler {
    pub runqueues: Vec<DeadlineRunQueue>,
    pub global_stats: DlStats,
    pub enabled: bool,
}

impl Default for DlScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl DlScheduler {
    pub fn new() -> Self {
        Self {
            runqueues: vec![DeadlineRunQueue::new(0)],
            global_stats: DlStats::default(),
            enabled: true,
        }
    }
    
    /// Создать новый ранкью для CPU
    pub fn add_cpu(&mut self, cpu_id: usize) {
        self.runtimes.push(DeadlineRunQueue::new(cpu_id));
    }
    
    /// Удалить ранкью
    pub fn remove_cpu(&mut self, cpu_id: usize) {
        if let Some(pos) = self.runtimes.iter().position(|r| r.cpu_id == cpu_id) {
            self.runtimes.remove(pos);
        }
    }
    
    /// Добавить deadline задачу
    pub fn dl_enqueue_task(&mut self, cpu_id: usize, task: DlTask) -> Result<(), SchedError> {
        if let Some(rq) = self.runtimes.get_mut(cpu_id) {
            rq.enqueue_task(task)
        } else {
            Err(SchedError::InvalidCpuId)
        }
    }
    
    /// Получить следующую deadline задачу
    pub fn dl_pick_next_task(&mut self, cpu_id: usize) -> Option<DlTask> {
        self.runtimes.get_mut(cpu_id)?.pick_next_task().map(|t| t.clone())
    }
    
    /// Удалить deadline задачу
    pub fn dl_dequeue_task(&mut self, cpu_id: usize, pid: u32) -> Result<Option<DlTask>, SchedError> {
        if let Some(rq) = self.runtimes.get_mut(cpu_id) {
            rq.dequeue_task(pid)
        } else {
            Err(SchedError::InvalidCpuId)
        }
    }
    
    /// Обработка тика таймера
    pub fn dl_tick(&mut self, cpu_id: usize, current_time: u64) -> Option<u32> {
        self.global_stats.tick();
        
        if let Some(rq) = self.runtimes.get_mut(cpu_id) {
            if let Some(current) = rq.pick_next_task() {
                // Проверяем дедлайн
                if current.is_deadline_expired(current_time) {
                    self.global_stats.on_missed_deadline();
                }
                
                // Уменьшаем runtime
                current.consume_runtime(1);
                
                // Если runtime кончился, обновляем дедлайн
                if !current.has_runtime() {
                    current.update_deadline(current_time);
                    // Перемещаем задачу в карту с новым дедлайном
                    let old_deadline = current.deadline_ptr - current.params.period;
                    self.runtimes[cpu_id].tasks.remove(&old_deadline);
                    self.runtimes[cpu_id].tasks.insert(current.deadline_ptr, current.clone());
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
    
    /// Включение/выключение
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    /// Проверка есть ли deadline задачи
    pub fn has_dl_tasks(&self, cpu_id: usize) -> bool {
        self.runtimes.get(cpu_id)
            .map(|rq| rq.count_tasks() > 0)
            .unwrap_or(false)
    }
    
    /// Глобальная балансировка нагрузки между CPU
    pub fn balance_load(&mut self) {
        // Простая балансировка: перемещаем задачи с перегруженных CPU
        if self.runtimes.len() < 2 {
            return;
        }
        
        let mut max_rq_idx = 0;
        let mut min_rq_idx = 0;
        let mut max_tasks = 0;
        let mut min_tasks = usize::MAX;
        
        for (idx, rq) in self.runtimes.iter().enumerate() {
            if rq.count_tasks() > max_tasks {
                max_tasks = rq.count_tasks();
                max_rq_idx = idx;
            }
            if rq.count_tasks() < min_tasks {
                min_tasks = rq.count_tasks();
                min_rq_idx = idx;
            }
        }
        
        // Перемещаем одну задачу если разница > 1
        if max_tasks - min_tasks > 1 && max_rq_idx != min_rq_idx {
            if let (Some(max_rq), Some(min_rq)) = 
                (self.runtimes.get_mut(max_rq_idx), self.runtimes.get_mut(min_rq_idx))
            {
                if let Some(task) = max_rq.tasks.values().next().cloned() {
                    let old_deadline = task.deadline_ptr;
                    max_rq.tasks.remove(&old_deadline);
                    min_rq.tasks.insert(task.deadline_ptr, task);
                }
            }
        }
    }
}

/// Расчет Utlization Bound для Rate Monotonic Scheduling
pub fn utilization_bound(n: u32) -> f64 {
    if n == 0 {
        return 0.0;
    }
    (n as f64) * (2.0_f64.powf(1.0 / n as f64) - 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dl_params_validation() {
        let valid_params = DlParams {
            runtime: 500_000_000,
            period: 1_000_000_000,
            deadline: 1_000_000_000,
        };
        assert!(valid_params.validate().is_ok());
        
        let invalid_params = DlParams {
            runtime: 0,
            period: 1_000_000_000,
            deadline: 1_000_000_000,
        };
        assert!(invalid_params.validate().is_err());
    }
    
    #[test]
    fn test_dl_task_deadline_update() {
        let params = DlParams::default();
        let mut task = DlTask {
            pid: 1,
            name: "test".to_string(),
            params: params.clone(),
            deadline_ptr: 0,
            last_runtime: 0,
            curr_runtime: 100,
            flags: DlTaskFlags::empty(),
        };
        
        task.update_deadline(1000);
        assert!(task.deadline_ptr > 0);
        assert_eq!(task.curr_runtime, params.runtime);
    }
    
    #[test]
    fn test_dl_queue_earliest_deadline_first() {
        let mut queue = DeadlineRunQueue::new(0);
        
        let mut task1 = DlTask::from_task(&Task { pid: 1, ..Default::default() }, DlParams {
            runtime: 100, period: 1000, deadline: 1000
        }).unwrap();
        task1.deadline_ptr = 1000;
        
        let mut task2 = DlTask::from_task(&Task { pid: 2, ..Default::default() }, DlParams {
            runtime: 100, period: 1000, deadline: 1000
        }).unwrap();
        task2.deadline_ptr = 500; // Более ранний дедлайн
        
        queue.enqueue_task(task1).unwrap();
        queue.enqueue_task(task2).unwrap();
        
        // Task2 должна быть выбрана первой (дедлайн 500 < 1000)
        let next = queue.pick_next_task().unwrap();
        assert_eq!(next.pid, 2);
    }
    
    #[test]
    fn test_utilization_bound() {
        assert!((utilization_bound(1) - 0.693).abs() < 0.001);
        assert!((utilization_bound(2) - 0.828).abs() < 0.001);
        assert!((utilization_bound(10) - 0.718).abs() < 0.001);
        // При n -> infinity, bound -> ln(2) ≈ 0.693
        assert!((utilization_bound(100) - 0.695).abs() < 0.001);
    }
}
