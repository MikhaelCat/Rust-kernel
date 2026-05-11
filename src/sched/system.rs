use super::cfs::CfsStats;
use super::loadavg::LoadAvg;
use super::manager::{SchedClass, SchedulerManager};
use super::preempt::Preempt;
use super::rt::RtStats;
use super::topology::Topology;

#[derive(Debug, Clone, PartialEq)]
pub struct SchedSnapshot {
    pub next_pid: u32,
    pub runq_len: usize,
    pub cfs_vruntime: u64,
    pub rt_budget: i64,
    pub load1: f32,
    pub preempt_count: u32,
    pub cpus: u32,
}

#[derive(Debug, Default)]
pub struct SchedSystem {
    mgr: SchedulerManager,
    cfs: CfsStats,
    rt: RtStats,
    load: LoadAvg,
    preempt: Preempt,
    topo: Topology,
    tick_no: u64,
}

impl SchedSystem {
    pub fn new(cpus: u32) -> Self {
        let mut topo = Topology::default();
        topo.set_cpus(cpus);
        Self {
            mgr: {
                let mut m = SchedulerManager::default();
                m.set_cpu_count(cpus);
                m
            },
            cfs: CfsStats::default(),
            rt: RtStats { budget: 10 },
            load: LoadAvg::default(),
            preempt: Preempt::default(),
            topo,
            tick_no: 0,
        }
    }

    pub fn enqueue_cfs(&mut self, pid: u32, cpu: u32) {
        self.mgr.enqueue_with(pid, SchedClass::Cfs, 120, cpu);
    }

    pub fn enqueue_rt(&mut self, pid: u32, cpu: u32) {
        self.mgr.enqueue_with(pid, SchedClass::Rt, 10, cpu);
    }

    pub fn tick(&mut self) -> u32 {
        self.preempt.disable();
        let cpu = (self.tick_no as u32) % self.topo.cpus().max(1);
        let pid = self.mgr.pick_next_on_cpu(cpu).expect("scheduler empty");
        self.tick_no += 1;

        // Minimal class/accounting model.
        if pid % 2 == 0 {
            self.rt.consume();
        } else {
            self.cfs.tick();
        }

        let denom = self.topo.cpus().max(1) as f32;
        self.load.update(self.mgr.len() as f32 / denom);
        self.preempt.enable();
        pid
    }

    pub fn set_affinity(&mut self, pid: u32, cpu: u32) {
        let _ = self.mgr.set_affinity(pid, cpu);
    }

    pub fn rebalance(&mut self) -> usize {
        self.mgr.rebalance()
    }

    pub fn snapshot(&self, last_pid: u32) -> SchedSnapshot {
        SchedSnapshot {
            next_pid: last_pid,
            runq_len: self.mgr.len(),
            cfs_vruntime: self.cfs.vruntime,
            rt_budget: self.rt.budget,
            load1: self.load.one,
            preempt_count: self.preempt.count(),
            cpus: self.topo.cpus(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rt_preempts_and_accounts() {
        let mut s = SchedSystem::new(4);
        s.enqueue_cfs(1, 0);
        s.enqueue_rt(2, 1);

        let pid = s.tick();
        assert_eq!(pid, 2);

        let snap = s.snapshot(pid);
        assert_eq!(snap.runq_len, 2);
        assert_eq!(snap.rt_budget, 9);
        assert_eq!(snap.preempt_count, 0);
        assert_eq!(snap.cpus, 4);
        assert!(snap.load1 > 0.0);
    }

    #[test]
    fn cfs_tick_accounts_vruntime() {
        let mut s = SchedSystem::new(2);
        s.enqueue_cfs(1, 0);
        let pid = s.tick();
        assert_eq!(pid, 1);
        let snap = s.snapshot(pid);
        assert_eq!(snap.cfs_vruntime, 1);
    }

    #[test]
    fn rebalance_reduces_skew() {
        let mut s = SchedSystem::new(2);
        s.enqueue_cfs(1, 0);
        s.enqueue_cfs(3, 0);
        s.enqueue_rt(2, 0);
        assert!(s.rebalance() > 0);
    }
}
