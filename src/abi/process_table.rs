use std::collections::BTreeMap;

use super::fs::FileTable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessEntry {
    pub pid: i64,
    pub ppid: i64,
}

#[derive(Debug)]
pub struct ProcessTable {
    current_pid: i64,
    next_pid: i64,
    procs: BTreeMap<i64, ProcessEntry>,
    fds: BTreeMap<i64, FileTable>,
}

impl ProcessTable {
    pub fn new(init_pid: i64) -> Self {
        let mut procs = BTreeMap::new();
        procs.insert(
            init_pid,
            ProcessEntry {
                pid: init_pid,
                ppid: 0,
            },
        );

        let mut fds = BTreeMap::new();
        fds.insert(init_pid, FileTable::new());

        Self {
            current_pid: init_pid,
            next_pid: init_pid + 1,
            procs,
            fds,
        }
    }

    pub fn current_pid(&self) -> i64 {
        self.current_pid
    }

    pub fn switch_to(&mut self, pid: i64) -> bool {
        if self.procs.contains_key(&pid) {
            self.current_pid = pid;
            true
        } else {
            false
        }
    }

    pub fn fork_like(&mut self) -> i64 {
        let child = self.next_pid;
        self.next_pid += 1;
        self.procs.insert(
            child,
            ProcessEntry {
                pid: child,
                ppid: self.current_pid,
            },
        );
        self.fds.insert(child, FileTable::new());
        child
    }

    pub fn file_table_mut(&mut self) -> &mut FileTable {
        self.fds
            .get_mut(&self.current_pid)
            .expect("current pid must always have fd table")
    }

    pub fn has_pid(&self, pid: i64) -> bool {
        self.procs.contains_key(&pid)
    }
}
