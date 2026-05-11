#[derive(Debug, Default)]
pub struct Compaction {
    pub runs: u64,
}
impl Compaction {
    pub fn run(&mut self) {
        self.runs += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_compaction() {
        let mut c = Compaction::default();
        c.run();
        assert_eq!(c.runs, 1);
    }
}
