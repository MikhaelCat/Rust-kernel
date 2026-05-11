use crate::system_events::SystemEvent;

pub fn can_start_kernel(events: &[SystemEvent]) -> bool {
    events.contains(&SystemEvent::BootReady)
        && events.contains(&SystemEvent::ArchReady)
        && events.contains(&SystemEvent::MmReady)
}

pub fn validate_runtime_order(events: &[SystemEvent]) -> bool {
    use SystemEvent::*;
    let expected = [
        BootStarted,
        BootReady,
        ArchReady,
        PowerReady,
        TimeReady,
        DriversReady,
        FsReady,
        MmReady,
        KernelReady,
        NetReady,
        IpcReady,
        SecurityReady,
        SystemReady,
    ];

    // ordered subsequence check
    let mut idx = 0usize;
    for ev in events {
        if *ev == expected[idx] {
            idx += 1;
            if idx == expected.len() {
                return true;
            }
        }
    }
    false
}

pub fn has_single_profile(events: &[SystemEvent]) -> bool {
    use SystemEvent::*;
    let mut count = 0usize;
    for ev in events {
        if matches!(ev, ProfileNormal | ProfileSafe | ProfileRecovery) {
            count += 1;
        }
    }
    count == 1
}

pub fn no_duplicate_critical(events: &[SystemEvent]) -> bool {
    use SystemEvent::*;
    let critical = [
        BootStarted,
        BootReady,
        ArchReady,
        MmReady,
        KernelReady,
        SystemReady,
    ];
    critical
        .iter()
        .all(|ev| events.iter().filter(|v| *v == ev).count() == 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guard_requires_prerequisites() {
        let ev = [SystemEvent::BootReady, SystemEvent::ArchReady];
        assert!(!can_start_kernel(&ev));

        let ev2 = [
            SystemEvent::BootReady,
            SystemEvent::ArchReady,
            SystemEvent::MmReady,
        ];
        assert!(can_start_kernel(&ev2));
    }

    #[test]
    fn runtime_order_validation() {
        let ok = [
            SystemEvent::BootStarted,
            SystemEvent::BootReady,
            SystemEvent::ArchReady,
            SystemEvent::PowerReady,
            SystemEvent::TimeReady,
            SystemEvent::DriversReady,
            SystemEvent::FsReady,
            SystemEvent::MmReady,
            SystemEvent::KernelReady,
            SystemEvent::NetReady,
            SystemEvent::IpcReady,
            SystemEvent::SecurityReady,
            SystemEvent::SystemReady,
        ];
        assert!(validate_runtime_order(&ok));

        let bad = [
            SystemEvent::BootStarted,
            SystemEvent::ArchReady,
            SystemEvent::BootReady,
            SystemEvent::SystemReady,
        ];
        assert!(!validate_runtime_order(&bad));
    }

    #[test]
    fn profile_must_be_unique() {
        let ok = [SystemEvent::BootStarted, SystemEvent::ProfileNormal];
        assert!(has_single_profile(&ok));

        let bad = [SystemEvent::ProfileNormal, SystemEvent::ProfileSafe];
        assert!(!has_single_profile(&bad));
    }

    #[test]
    fn critical_events_must_not_repeat() {
        let ok = [
            SystemEvent::BootStarted,
            SystemEvent::BootReady,
            SystemEvent::ArchReady,
            SystemEvent::MmReady,
            SystemEvent::KernelReady,
            SystemEvent::SystemReady,
        ];
        assert!(no_duplicate_critical(&ok));

        let bad = [
            SystemEvent::BootStarted,
            SystemEvent::BootStarted,
            SystemEvent::BootReady,
            SystemEvent::ArchReady,
            SystemEvent::MmReady,
            SystemEvent::KernelReady,
            SystemEvent::SystemReady,
        ];
        assert!(!no_duplicate_critical(&bad));
    }
}
