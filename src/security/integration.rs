use crate::system_events::SystemEvent;

pub fn security_may_enforce(events: &[SystemEvent]) -> bool {
    events.contains(&SystemEvent::KernelReady)
        && events.contains(&SystemEvent::NetReady)
        && events.contains(&SystemEvent::IpcReady)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enforce_after_core_ready() {
        let ev = [SystemEvent::KernelReady, SystemEvent::NetReady];
        assert!(!security_may_enforce(&ev));

        let ev2 = [
            SystemEvent::KernelReady,
            SystemEvent::NetReady,
            SystemEvent::IpcReady,
        ];
        assert!(security_may_enforce(&ev2));
    }
}
