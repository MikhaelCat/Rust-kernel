#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicInfo {
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PanicReport {
    pub reason: &'static str,
    pub severity: &'static str,
}

pub fn panic_now(reason: &'static str) -> PanicInfo {
    PanicInfo { reason }
}

pub fn panic_report(reason: &'static str) -> PanicReport {
    let severity = if reason.contains("memory") || reason.contains("corruption") {
        "critical"
    } else {
        "high"
    };

    PanicReport { reason, severity }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panic_reason() {
        assert_eq!(panic_now("oops").reason, "oops");
    }

    #[test]
    fn panic_report_severity() {
        assert_eq!(panic_report("memory corruption").severity, "critical");
        assert_eq!(panic_report("scheduler stall").severity, "high");
    }
}
