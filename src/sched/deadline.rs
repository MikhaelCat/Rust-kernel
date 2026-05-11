#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadlineTask {
    pub runtime: u64,
    pub deadline: u64,
}
impl DeadlineTask {
    pub fn new(runtime: u64, deadline: u64) -> Self {
        Self { runtime, deadline }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deadline_new() {
        assert_eq!(DeadlineTask::new(1, 10).deadline, 10);
    }
}
