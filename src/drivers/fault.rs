#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverFault {
    Recoverable,
    Fatal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultAction {
    Retry,
    Reset,
    Quarantine,
}

pub fn handle_fault(f: DriverFault) -> FaultAction {
    match f {
        DriverFault::Recoverable => FaultAction::Retry,
        DriverFault::Fatal => FaultAction::Quarantine,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fault_mapping() {
        assert_eq!(handle_fault(DriverFault::Recoverable), FaultAction::Retry);
        assert_eq!(handle_fault(DriverFault::Fatal), FaultAction::Quarantine);
    }
}
