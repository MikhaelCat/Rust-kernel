#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompAction {
    Allow,
    Kill,
}
pub fn filter_syscall(num: u64) -> SeccompAction {
    if num == 59 {
        SeccompAction::Kill
    } else {
        SeccompAction::Allow
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn seccomp_filter() {
        assert_eq!(filter_syscall(59), SeccompAction::Kill);
    }
}
