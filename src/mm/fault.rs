#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFault {
    NotPresent,
    Protection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultAction {
    MapPage,
    KillTask,
}

pub fn handle_fault(f: PageFault) -> FaultAction {
    match f {
        PageFault::NotPresent => FaultAction::MapPage,
        PageFault::Protection => FaultAction::KillTask,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fault_actions() {
        assert_eq!(handle_fault(PageFault::NotPresent), FaultAction::MapPage);
        assert_eq!(handle_fault(PageFault::Protection), FaultAction::KillTask);
    }
}
