use rust_linux_kernel::runtime_config::RuntimeConfig;
use rust_linux_kernel::self_check::run_self_check;
use rust_linux_kernel::system::RustLinuxSystem;
use rust_linux_kernel::tools::file_report::write_report;
use rust_linux_kernel::tools::health_matrix::HealthMatrix;
use rust_linux_kernel::tools::report::render_from_snapshot;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cfg = RuntimeConfig::from_env_and_args(&args);

    let mut self_report = None;
    if cfg.self_check {
        let report = run_self_check();
        println!(
            "self-check: runtime_ok={}, fault_ok={}, all_ok={}",
            report.runtime_ok,
            report.fault_ok,
            report.all_ok()
        );
        if !report.all_ok() {
            std::process::exit(2);
        }
        self_report = Some(report);
    }

    let mut sys = RustLinuxSystem::new();
    let report = sys.boot_with_profile(cfg.profile);

    if cfg.verbose {
        println!("events={:?}", sys.events());
        if let Some(sr) = &self_report {
            let p = render_from_snapshot(sr);
            println!("probe={} details={}", p.overall_ok, p.details);

            let matrix = HealthMatrix::from_self_check(sr);
            let txt = matrix.to_text();
            let _ = write_report(std::path::Path::new("target/self-check-matrix.txt"), &txt);
        }
    }

    println!(
        "rust-linux integrated: profile={:?}, boot_ok={}, arch_ok={}, pid={}, net_bytes={}, ipc_ok={}, sec_ok={}",
        report.profile,
        report.boot_ok,
        report.arch_ok,
        report.pid,
        report.net_bytes,
        report.ipc_ok,
        report.sec_ok
    );
}
