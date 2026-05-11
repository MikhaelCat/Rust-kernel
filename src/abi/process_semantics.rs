use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcState {
    Running,
    Exited(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcMeta {
    pub pid: i64,
    pub ppid: i64,
    pub name: String,
    pub state: ProcState,
}

#[derive(Debug)]
pub struct ProcSemantics {
    current: i64,
    next: i64,
    procs: BTreeMap<i64, ProcMeta>,
    zombie_queue: VecDeque<(i64, i64)>,
}

impl ProcSemantics {
    pub fn new(init_pid: i64) -> Self {
        let mut procs = BTreeMap::new();
        procs.insert(
            init_pid,
            ProcMeta {
                pid: init_pid,
                ppid: 0,
                name: "init".to_string(),
                state: ProcState::Running,
            },
        );
        Self {
            current: init_pid,
            next: init_pid + 1,
            procs,
            zombie_queue: VecDeque::new(),
        }
    }

    pub fn current_pid(&self) -> i64 {
        self.current
    }

    pub fn fork_like(&mut self) -> i64 {
        let child = self.next;
        self.next += 1;
        self.procs.insert(
            child,
            ProcMeta {
                pid: child,
                ppid: self.current,
                name: "child".to_string(),
                state: ProcState::Running,
            },
        );
        child
    }

    pub fn switch_to(&mut self, pid: i64) -> bool {
        if matches!(
            self.procs.get(&pid),
            Some(ProcMeta {
                state: ProcState::Running,
                ..
            })
        ) {
            self.current = pid;
            true
        } else {
            false
        }
    }

    pub fn exec_like(&mut self, new_name: &str) -> bool {
        if let Some(p) = self.procs.get_mut(&self.current) {
            p.name = new_name.to_string();
            true
        } else {
            false
        }
    }

    pub fn exit_like(&mut self, code: i64) -> bool {
        if let Some(p) = self.procs.get_mut(&self.current) {
            p.state = ProcState::Exited(code);
            self.zombie_queue.push_back((p.pid, code));
            true
        } else {
            false
        }
    }

    pub fn wait_like(&mut self) -> Option<(i64, i64)> {
        self.zombie_queue.pop_front()
    }

    pub fn waitpid_like(&mut self, wanted: i64) -> Option<(i64, i64)> {
        if wanted == -1 {
            return self.wait_like();
        }
        let idx = self
            .zombie_queue
            .iter()
            .position(|(pid, _)| *pid == wanted)?;
        self.zombie_queue.remove(idx)
    }

    pub fn has_pid(&self, pid: i64) -> bool {
        self.procs.contains_key(&pid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fork_exec_wait_flow() {
        let mut ps = ProcSemantics::new(1);
        let child = ps.fork_like();
        assert!(ps.switch_to(child));
        assert!(ps.exec_like("worker"));
        assert!(ps.exit_like(0));
        assert_eq!(ps.wait_like(), Some((child, 0)));
    }

    #[test]
    fn waitpid_specific_child() {
        let mut ps = ProcSemantics::new(1);
        let c1 = ps.fork_like();
        let c2 = ps.fork_like();
        assert!(ps.switch_to(c1));
        assert!(ps.exit_like(11));
        assert!(ps.switch_to(c2));
        assert!(ps.exit_like(22));
        assert_eq!(ps.waitpid_like(c2), Some((c2, 22)));
        assert_eq!(ps.waitpid_like(c1), Some((c1, 11)));
    }
}
