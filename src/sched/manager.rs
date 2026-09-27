//! Scheduler Manager - Coordinates all scheduling algorithms
//!
//! Интегрирует CFS, RT, и Deadline планировщики в единую систему

use super::cfs::{CfsRunQueue, NiceToWeight};
use super::rt::{RtScheduler, RtTask};
use super::deadline::{DlParams, DlTask, DeadlineRunQueue, DlScheduler};
use super::types::Task;
use std::collections::{VecDeque, BTreeMap};

#[derive(Debug, Clone)]
pub enum PolicyType {
    Cfs,
    Fifo,
    RoundRobin,
    Deadline,
}

#[derive(Debug)]
pub struct SchedulerManager {
    // CFS Run Queues per CPU
    pub cfs_rqs: Vec<CfsRunQueue>,
    
    // RT Scheduler
    pub rt_scheduler: RtScheduler,
    
    // Deadline Scheduler  
    pub dl_scheduler: DlScheduler,
    
    // Global statistics
    pub context_switches: u64,
    pub total_runtime_ns: u64,
    
    // Last executed task info
    pub last_exec_task: Option<TaskInfo>,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub pid: u32,
    pub comm: String,
    pub policy: PolicyType,
}

impl Default for SchedulerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SchedulerManager {
    pub fn new() -> Self {
        Self {
            cfs_rqs: vec![CfsRunQueue::new(0)],
            rt_scheduler: RtScheduler::new(),
            dl_scheduler: DlScheduler::new(),
            context_switches: 0,
            total_runtime_ns: 0,
            last_exec_task: None,
        }
    }

    /// Добавить задачу в правильный планировщик
    pub fn enqueue_task(&mut self, task: &mut Task) {
        // Определяем тип задачи по приоритету
        if task.rt_prio.as_u8() > 0 {
            // Real-time задача (SCHED_FIFO или SCHED_RR)
            let cpu_id = self.select_cpu_for_rt();
            if let Ok(_) = self.rt_scheduler.rt_enqueue_task(cpu_id, task) {
                return;
            }
        }
        
        // Для нормальных CFS задач
        let cpu_id = self.select_cpu_for_cfs();
        if let Some(rq) = self.cfs_rqs.get_mut(cpu_id) {
            let _ = rq.enqueue_task(task);
        }
    }

    /// Выбор следующей задачи для выполнения
    pub fn select_next_task(&mut self, cpu_id: usize) -> Option<u32> {
        // Приоритет RT > Deadline > CFS
        
        // Проверяем RT задачи
        if self.rt_scheduler.has_rt_tasks(cpu_id) {
            if let Some(pid) = self.rt_scheduler.rt_tick(cpu_id, 0) {
                return Some(pid);
            }
        }
        
        // Проверяем Deadline задачи  
        if self.dl_scheduler.has_dl_tasks(cpu_id) {
            use std::time::Instant;
            let now = Instant::now().elapsed().as_nanos() as u64;
            if let Some(pid) = self.dl_scheduler.dl_tick(cpu_id, now) {
                return Some(pid);
            }
        }
        
        // CFS задачи
        if let Some(rq) = self.cfs_rqs.get_mut(cpu_id) {
            if let Some(cfs_task) = rq.pick_next_task() {
                let pid = cfs_task.ts.pid;
                self.last_exec_task = Some(TaskInfo {
                    pid,
                    comm: cfs_task.ts.comm.clone(),
                    policy: PolicyType::Cfs,
                });
                return Some(pid);
            }
        }
        
        None
    }

    /// Обработка тика таймера для текущей задачи
    pub fn tick(&mut self, cpu_id: usize, current_pid: u32) {
        // Обновляем статистику
        self.context_switches += 1;
        self.total_runtime_ns += 1_000_000; // ~1ms тик
        
        // CFS tick
        if let Some(rq) = self.cfs_rqs.get_mut(cpu_id) {
            rq.task_tick(current_pid);
        }
        
        // RT tick  
        self.rt_scheduler.rt_tick(cpu_id, current_pid);
        
        // Deadline tick (с временной меткой)
        use std::time::Instant;
        let now = Instant::now().elapsed().as_nanos() as u64;
        self.dl_scheduler.dl_tick(cpu_id, now);
    }

    pub fn cpu_count(&self) -> usize {
        self.cfs_rqs.len()
    }
    
    /// Выбор CPU для RT задач
    pub fn select_cpu_for_rt(&self) -> usize {
        0.min(self.rt_scheduler.cpu_count() - 1)
    }
    
    /// Выбор CPU для CFS задач
    pub fn select_cpu_for_cfs(&self) -> usize {
        0.min(self.cfs_rqs.len() - 1)
    }
}
