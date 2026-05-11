#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    pub seq: u64,
    pub op: String,
    pub path: String,
}

#[derive(Debug, Default)]
pub struct Journal {
    txns: u64,
    entries: Vec<JournalEntry>,
}
impl Journal {
    pub fn begin(&mut self) {
        self.txns += 1;
    }
    pub fn txns(&self) -> u64 {
        self.txns
    }

    pub fn record(&mut self, op: &str, path: &str) {
        self.begin();
        self.entries.push(JournalEntry {
            seq: self.txns,
            op: op.to_string(),
            path: path.to_string(),
        });
    }

    pub fn entries(&self) -> &[JournalEntry] {
        &self.entries
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn begin_txn() {
        let mut j = Journal::default();
        j.begin();
        assert_eq!(j.txns(), 1);
    }

    #[test]
    fn record_entry() {
        let mut j = Journal::default();
        j.record("create", "/tmp/a");
        assert_eq!(j.txns(), 1);
        assert_eq!(j.entries().len(), 1);
    }
}
