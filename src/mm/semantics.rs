use super::fault::{FaultAction, PageFault, handle_fault};
use super::protection::{AccessType, ProtFlags, check_access};

pub fn run_mm_semantics() -> bool {
    let rw = ProtFlags::rw();
    let rx = ProtFlags::rx();

    let p1 = check_access(rw, AccessType::Read)
        && check_access(rw, AccessType::Write)
        && !check_access(rw, AccessType::Exec);

    let p2 = check_access(rx, AccessType::Read)
        && !check_access(rx, AccessType::Write)
        && check_access(rx, AccessType::Exec);

    let f1 = handle_fault(PageFault::NotPresent) == FaultAction::MapPage;
    let f2 = handle_fault(PageFault::Protection) == FaultAction::KillTask;

    p1 && p2 && f1 && f2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mm_semantics_ok() {
        assert!(run_mm_semantics());
    }
}
