#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageOwner {
    pub pid: u32,
}
impl PageOwner {
    pub fn new(pid: u32) -> Self {
        Self { pid }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn owner_pid() {
        assert_eq!(PageOwner::new(42).pid, 42);
    }
}
