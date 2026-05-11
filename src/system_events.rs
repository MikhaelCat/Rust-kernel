#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemEvent {
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
    ProfileNormal,
    ProfileSafe,
    ProfileRecovery,
}

#[derive(Debug, Default)]
pub struct EventBus {
    events: Vec<SystemEvent>,
}

impl EventBus {
    pub fn emit(&mut self, ev: SystemEvent) {
        self.events.push(ev);
    }

    pub fn emit_once(&mut self, ev: SystemEvent) {
        if !self.contains(ev) {
            self.emit(ev);
        }
    }

    pub fn contains(&self, ev: SystemEvent) -> bool {
        self.events.contains(&ev)
    }

    pub fn all(&self) -> &[SystemEvent] {
        &self.events
    }

    pub fn last(&self) -> Option<SystemEvent> {
        self.events.last().copied()
    }

    pub fn count(&self) -> usize {
        self.events.len()
    }

    pub fn contains_in_order(&self, expected: &[SystemEvent]) -> bool {
        if expected.is_empty() {
            return true;
        }

        let mut idx = 0usize;
        for ev in &self.events {
            if *ev == expected[idx] {
                idx += 1;
                if idx == expected.len() {
                    return true;
                }
            }
        }

        false
    }

    pub fn profile_selected(&self) -> bool {
        self.contains(SystemEvent::ProfileNormal)
            || self.contains(SystemEvent::ProfileSafe)
            || self.contains(SystemEvent::ProfileRecovery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_recording() {
        let mut b = EventBus::default();
        b.emit(SystemEvent::BootStarted);
        assert!(b.contains(SystemEvent::BootStarted));
        assert_eq!(b.count(), 1);
        assert_eq!(b.last(), Some(SystemEvent::BootStarted));
    }

    #[test]
    fn verifies_ordered_subsequence() {
        let mut b = EventBus::default();
        b.emit(SystemEvent::BootStarted);
        b.emit(SystemEvent::BootReady);
        b.emit(SystemEvent::ArchReady);
        b.emit(SystemEvent::SystemReady);

        assert!(b.contains_in_order(&[
            SystemEvent::BootStarted,
            SystemEvent::ArchReady,
            SystemEvent::SystemReady,
        ]));

        assert!(!b.contains_in_order(&[SystemEvent::ArchReady, SystemEvent::BootStarted,]));
    }

    #[test]
    fn emit_once_avoids_duplicates() {
        let mut b = EventBus::default();
        b.emit_once(SystemEvent::BootReady);
        b.emit_once(SystemEvent::BootReady);
        assert_eq!(b.count(), 1);
    }
}
