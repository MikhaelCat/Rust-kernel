use crate::self_check::run_self_check;
use crate::tools::health_matrix::HealthMatrix;
use crate::tools::report::render_from_snapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemDiagReport {
    pub matrix_ok: bool,
    pub report_ok: bool,
    pub lines: usize,
}

pub fn run_system_diag() -> SystemDiagReport {
    let s = run_self_check();
    let m = HealthMatrix::from_self_check(&s);
    let p = render_from_snapshot(&s);
    let lines = m.to_text().lines().count();
    SystemDiagReport {
        matrix_ok: m.all_ok(),
        report_ok: p.overall_ok,
        lines,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diag_report_ok() {
        let d = run_system_diag();
        assert!(d.matrix_ok);
        assert!(d.report_ok);
        assert!(d.lines >= 10);
    }
}
