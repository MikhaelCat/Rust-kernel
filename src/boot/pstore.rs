#[derive(Debug, Default)]
pub struct Pstore {
    records: u64,
}
impl Pstore {
    pub fn append(&mut self) {
        self.records += 1;
    }
    pub fn records(&self) -> u64 {
        self.records
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn append_record() {
        let mut p = Pstore::default();
        p.append();
        assert_eq!(p.records(), 1);
    }
}
