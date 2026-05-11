use crate::self_check::SelfCheckReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeReport {
    pub overall_ok: bool,
    pub details: String,
}

pub fn render_from_snapshot(r: &SelfCheckReport) -> ProbeReport {
    ProbeReport {
        overall_ok: r.all_ok(),
        details: format!(
            "runtime={} boot={} fault={} abi={} sched={} platform={} mm={} deep={} sec={} rep={}",
            r.runtime_ok,
            r.boot_ok,
            r.fault_ok,
            r.abi_ok,
            r.sched_ok,
            r.platform_ok,
            r.mm_prot_ok,
            r.deep_ok,
            r.security_bundle_ok,
            r.report_ok
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_render_non_empty() {
        let s = SelfCheckReport {
            runtime_ok: true,
            boot_ok: true,
            fault_ok: true,
            abi_ok: true,
            sched_ok: true,
            platform_ok: true,
            mm_prot_ok: true,
            deep_ok: true,
            security_bundle_ok: true,
            report_ok: true,
            parity_ok: true,
        };
        let r = render_from_snapshot(&s);
        assert!(r.overall_ok);
        assert!(r.details.contains("runtime=true"));
    }
}
