use std::collections::{BTreeMap, VecDeque};

use super::error::SchedError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedClass {
    Rt,
    Cfs,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedEntity {
    pub pid: u32,
    pub class: SchedClass,
    pub prio: u8,
    pub cpu: u32,
}

#[derive(Debug)]
pub struct SchedulerManager {
    rt_q: VecDeque<SchedEntity>,
    cfs_q: VecDeque<SchedEntity>,
    cpu_affinity: BTreeMap<u32, u32>,
    cpu_count: u32,
}

impl SchedulerManager {
    pub fn set_cpu_count(&mut self, cpus: u32) {
        self.cpu_count = cpus.max(1);
    }

    fn clamp_cpu(&self, cpu: u32) -> u32 {
        cpu % self.cpu_count.max(1)
    }

    pub fn enqueue(&mut self, pid: u32) {
        self.enqueue_with(pid, SchedClass::Cfs, 120, 0);
    }

    pub fn enqueue_with(&mut self, pid: u32, class: SchedClass, prio: u8, cpu: u32) {
        let cpu = self.clamp_cpu(cpu);
        let ent = SchedEntity {
            pid,
            class,
            prio,
            cpu,
        };
        self.cpu_affinity.insert(pid, cpu);
        match class {
            SchedClass::Rt => self.rt_q.push_back(ent),
            SchedClass::Cfs => self.cfs_q.push_back(ent),
        }
    }

    pub fn pick_next(&mut self) -> Result<u32, SchedError> {
        if let Some(ent) = self.rt_q.pop_front() {
            let pid = ent.pid;
            self.rt_q.push_back(ent);
            return Ok(pid);
        }
        if let Some(ent) = self.cfs_q.pop_front() {
            let pid = ent.pid;
            self.cfs_q.push_back(ent);
            return Ok(pid);
        }
        Err(SchedError::Empty)
    }

    pub fn pick_next_on_cpu(&mut self, cpu: u32) -> Result<u32, SchedError> {
        let cpu = self.clamp_cpu(cpu);
        if let Some(idx) = self.rt_q.iter().position(|e| e.cpu == cpu) {
            let ent = self.rt_q.remove(idx).expect("idx valid");
            let pid = ent.pid;
            self.rt_q.push_back(ent);
            return Ok(pid);
        }
        if let Some(ent) = self.rt_q.pop_front() {
            let pid = ent.pid;
            self.rt_q.push_back(ent);
            return Ok(pid);
        }
        if let Some(idx) = self.cfs_q.iter().position(|e| e.cpu == cpu) {
            let ent = self.cfs_q.remove(idx).expect("idx valid");
            let pid = ent.pid;
            self.cfs_q.push_back(ent);
            return Ok(pid);
        }
        if let Some(ent) = self.cfs_q.pop_front() {
            let pid = ent.pid;
            self.cfs_q.push_back(ent);
            return Ok(pid);
        }
        Err(SchedError::Empty)
    }

    pub fn set_affinity(&mut self, pid: u32, cpu: u32) -> Result<(), SchedError> {
        if !self.cpu_affinity.contains_key(&pid) {
            return Err(SchedError::Empty);
        }
        let cpu = self.clamp_cpu(cpu);
        self.cpu_affinity.insert(pid, cpu);

        for q in [&mut self.rt_q, &mut self.cfs_q] {
            for ent in q.iter_mut() {
                if ent.pid == pid {
                    ent.cpu = cpu;
                }
            }
        }
        Ok(())
    }

    pub fn affinity_of(&self, pid: u32) -> Option<u32> {
        self.cpu_affinity.get(&pid).copied()
    }

    pub fn len(&self) -> usize {
        self.rt_q.len() + self.cfs_q.len()
    }

    pub fn load_per_cpu(&self) -> BTreeMap<u32, usize> {
        let mut out = BTreeMap::new();
        for cpu in 0..self.cpu_count.max(1) {
            out.insert(cpu, 0usize);
        }
        for ent in self.rt_q.iter().chain(self.cfs_q.iter()) {
            *out.entry(ent.cpu).or_insert(0) += 1;
        }
        out
    }

    pub fn rebalance(&mut self) -> usize {
        let mut moved = 0usize;
        loop {
            let loads = self.load_per_cpu();
            let (max_cpu, max_load) = loads
                .iter()
                .max_by_key(|(_, load)| *load)
                .map(|(cpu, load)| (*cpu, *load))
                .expect("at least one cpu");
            let (min_cpu, min_load) = loads
                .iter()
                .min_by_key(|(_, load)| *load)
                .map(|(cpu, load)| (*cpu, *load))
                .expect("at least one cpu");
            if max_load <= min_load + 1 {
                break;
            }
            let mut changed = false;
            for q in [&mut self.cfs_q, &mut self.rt_q] {
                if let Some(ent) = q.iter_mut().find(|e| e.cpu == max_cpu) {
                    ent.cpu = min_cpu;
                    self.cpu_affinity.insert(ent.pid, min_cpu);
                    moved += 1;
                    changed = true;
                    break;
                }
            }
            if !changed {
                break;
            }
        }
        moved
    }
}

impl Default for SchedulerManager {
    fn default() -> Self {
        Self {
            rt_q: VecDeque::new(),
            cfs_q: VecDeque::new(),
            cpu_affinity: BTreeMap::new(),
            cpu_count: 64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rt_preempts_cfs() {
        let mut s = SchedulerManager::default();
        s.enqueue_with(10, SchedClass::Cfs, 120, 0);
        s.enqueue_with(20, SchedClass::Rt, 10, 0);
        assert_eq!(s.pick_next().expect("pick"), 20);
    }

    #[test]
    fn cfs_round_robin() {
        let mut s = SchedulerManager::default();
        s.enqueue(1);
        s.enqueue(2);
        assert_eq!(s.pick_next().expect("pick"), 1);
        assert_eq!(s.pick_next().expect("pick"), 2);
    }

    #[test]
    fn affinity_update() {
        let mut s = SchedulerManager::default();
        s.enqueue(7);
        s.set_affinity(7, 2).expect("affinity");
        assert_eq!(s.affinity_of(7), Some(2));
    }

    #[test]
    fn cpu_pick_prefers_local_queue() {
        let mut s = SchedulerManager::default();
        s.set_cpu_count(2);
        s.enqueue_with(10, SchedClass::Cfs, 120, 0);
        s.enqueue_with(20, SchedClass::Rt, 10, 1);
        assert_eq!(s.pick_next_on_cpu(1).expect("pick"), 20);
    }

    #[test]
    fn rebalance_moves_tasks() {
        let mut s = SchedulerManager::default();
        s.set_cpu_count(2);
        s.enqueue_with(1, SchedClass::Cfs, 120, 0);
        s.enqueue_with(2, SchedClass::Cfs, 120, 0);
        s.enqueue_with(3, SchedClass::Cfs, 120, 0);
        let moved = s.rebalance();
        assert!(moved > 0);
        let loads = s.load_per_cpu();
        let l0 = *loads.get(&0).expect("cpu0");
        let l1 = *loads.get(&1).expect("cpu1");
        assert!((l0 as i64 - l1 as i64).abs() <= 1);
    }
}
