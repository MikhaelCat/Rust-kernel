#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KThread {
    pub pid: u32,
    pub name: String,
}
impl KThread {
    pub fn new(pid: u32, name: &str) -> Self {
        Self {
            pid,
            name: name.to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn kthread_new() {
        assert_eq!(KThread::new(2, "kth").pid, 2);
    }
}
