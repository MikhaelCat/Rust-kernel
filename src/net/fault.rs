#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetFault {
    LinkDown,
    RouteMissing,
    MtuExceeded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetFaultAction {
    Retry,
    Reconfigure,
    Drop,
}

pub fn handle_net_fault(fault: NetFault) -> NetFaultAction {
    match fault {
        NetFault::LinkDown => NetFaultAction::Retry,
        NetFault::RouteMissing => NetFaultAction::Reconfigure,
        NetFault::MtuExceeded => NetFaultAction::Drop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_fault_actions() {
        assert_eq!(handle_net_fault(NetFault::LinkDown), NetFaultAction::Retry);
        assert_eq!(
            handle_net_fault(NetFault::RouteMissing),
            NetFaultAction::Reconfigure
        );
        assert_eq!(
            handle_net_fault(NetFault::MtuExceeded),
            NetFaultAction::Drop
        );
    }
}
