use crate::self_check::SelfCheckReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthRow {
    pub name: &'static str,
    pub ok: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HealthMatrix {
    pub rows: Vec<HealthRow>,
}

impl HealthMatrix {
    pub fn from_self_check(r: &SelfCheckReport) -> Self {
        Self {
            rows: vec![
                HealthRow {
                    name: "runtime",
                    ok: r.runtime_ok,
                },
                HealthRow {
                    name: "boot",
                    ok: r.boot_ok,
                },
                HealthRow {
                    name: "fault",
                    ok: r.fault_ok,
                },
                HealthRow {
                    name: "abi",
                    ok: r.abi_ok,
                },
                HealthRow {
                    name: "sched",
                    ok: r.sched_ok,
                },
                HealthRow {
                    name: "platform",
                    ok: r.platform_ok,
                },
                HealthRow {
                    name: "mm_prot",
                    ok: r.mm_prot_ok,
                },
                HealthRow {
                    name: "deep",
                    ok: r.deep_ok,
                },
                HealthRow {
                    name: "security_bundle",
                    ok: r.security_bundle_ok,
                },
                HealthRow {
                    name: "report",
                    ok: r.report_ok,
                },
                HealthRow {
                    name: "parity",
                    ok: r.parity_ok,
                },
            ],
        }
    }

    pub fn all_ok(&self) -> bool {
        self.rows.iter().all(|r| r.ok)
    }

    pub fn to_text(&self) -> String {
        let mut out = String::new();
        for r in &self.rows {
            out.push_str(&format!("{}={}\n", r.name, r.ok));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::self_check::run_self_check;

    #[test]
    fn matrix_from_self_check() {
        let r = run_self_check();
        let m = HealthMatrix::from_self_check(&r);
        assert!(m.all_ok());
        assert!(m.to_text().contains("runtime=true"));
    }
}
