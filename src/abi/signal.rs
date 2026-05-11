use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Term = 15,
    Kill = 9,
}

#[derive(Debug, Default)]
pub struct SignalTable {
    pending: BTreeMap<i64, Vec<Signal>>,
}

impl SignalTable {
    pub fn send(&mut self, pid: i64, sig: Signal) -> i64 {
        self.pending.entry(pid).or_default().push(sig);
        0
    }

    pub fn pending_count(&self, pid: i64) -> usize {
        self.pending.get(&pid).map(|v| v.len()).unwrap_or(0)
    }

    pub fn pop_pending(&mut self, pid: i64) -> Option<Signal> {
        let list = self.pending.get_mut(&pid)?;
        if list.is_empty() {
            return None;
        }
        Some(list.remove(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn send_signal() {
        let mut s = SignalTable::default();
        assert_eq!(s.send(1, Signal::Term), 0);
        assert_eq!(s.pending_count(1), 1);
    }
}
