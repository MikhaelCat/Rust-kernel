use crate::block::manager::BlockManager;
use crate::block::{bio::Bio, device::BlockDevice};
use crate::io_uring::{IoOp, IoUringEngine};
use crate::platform::{PlatformManager, board::Board};
use crate::sched::SchedulerManager;
use crate::syscall::{SyscallTable, sys_getpid};
use crate::tools::Diagnostics;
use crate::virt::manager::VirtManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysRuntimeReport {
    pub platform_ready: bool,
    pub block_inflight: usize,
    pub scheduled_pid: u32,
    pub getpid_value: i64,
    pub io_completed: bool,
    pub vm_started: bool,
    pub diagnostics_checks: u64,
    pub vm_healthy: bool,
    pub block_completed: usize,
    pub io_cqes: usize,
    pub vm_checkpoint_ok: bool,
    pub sched_cycles: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStageReport {
    pub name: &'static str,
    pub ok: bool,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysRuntimeScenario {
    pub summary: SysRuntimeReport,
    pub stages: Vec<RuntimeStageReport>,
}

impl SysRuntimeScenario {
    pub fn all_ok(&self) -> bool {
        self.stages.iter().all(|s| s.ok)
    }
}

pub fn run_runtime_path() -> SysRuntimeReport {
    run_runtime_scenario().summary
}

pub fn run_runtime_scenario() -> SysRuntimeScenario {
    let mut stages = Vec::new();

    let mut platform = PlatformManager::new(Board::generic());
    platform.load_firmware();
    let platform_ready = platform.boot_ready().is_ok();
    stages.push(RuntimeStageReport {
        name: "platform",
        ok: platform_ready,
        detail: "firmware loaded and platform ready".to_string(),
    });

    let mut blk = BlockManager::default();
    blk.attach_device(BlockDevice::new("vda", 512));
    let b0 = blk.submit_bio(Bio::new(0, 512)).is_ok();
    let b1 = blk.submit_bio(Bio::new(8, 4096)).is_ok();
    stages.push(RuntimeStageReport {
        name: "block_submit",
        ok: b0 && b1 && blk.inflight() == 2,
        detail: format!("inflight={}", blk.inflight()),
    });
    let block_completed = blk.complete_one().map(|_| 1).unwrap_or(0);
    stages.push(RuntimeStageReport {
        name: "block_complete",
        ok: block_completed == 1 && blk.inflight() == 1,
        detail: format!(
            "completed_now={}, inflight={}",
            block_completed,
            blk.inflight()
        ),
    });

    let mut sched = SchedulerManager::default();
    sched.enqueue(1);
    sched.enqueue(2);
    let mut scheduled_pid = 0u32;
    let mut sched_cycles = 0usize;
    for _ in 0..3 {
        if let Ok(pid) = sched.pick_next() {
            scheduled_pid = pid;
            sched_cycles += 1;
        }
    }
    stages.push(RuntimeStageReport {
        name: "sched",
        ok: sched_cycles == 3 && (scheduled_pid == 1 || scheduled_pid == 2),
        detail: format!("cycles={}, last_pid={}", sched_cycles, scheduled_pid),
    });

    let mut st = SyscallTable::new();
    st.register(39, sys_getpid);
    let getpid_value = st
        .invoke(39, &[scheduled_pid as u64])
        .expect("syscall failed");
    stages.push(RuntimeStageReport {
        name: "syscall_getpid",
        ok: getpid_value == scheduled_pid as i64,
        detail: format!("getpid={}", getpid_value),
    });

    let mut ring = IoUringEngine::default();
    let nop_id = ring.submit(IoOp::Nop);
    let _rd_id = ring.submit_block_read(4096);
    let _wr_id = ring.submit_block_write(4096);
    let c0 = ring.poll_once() == Some(nop_id);
    let batch_done = ring.complete_batch();
    let io_cqes = ring.cqe_count();
    let io_completed = c0 && batch_done >= 2 && io_cqes >= 3;
    stages.push(RuntimeStageReport {
        name: "io_uring",
        ok: io_completed,
        detail: format!("batch_done={}, cqes={}", batch_done, io_cqes),
    });

    let mut virt = VirtManager::default();
    let vm_id = virt.create_vm(512, [0, 1, 2, 3, 4, 5], 500_000);
    let vm_started = virt.boot_vm(vm_id);
    let _ = virt.balloon_vm(vm_id, 64);
    let vm_healthy = virt.vm_healthy(vm_id);
    let vm_checkpoint_ok = virt
        .checkpoint_vm(vm_id)
        .map(|cp| virt.restore_vm(&cp))
        .unwrap_or(false);
    stages.push(RuntimeStageReport {
        name: "virt",
        ok: vm_started && vm_healthy && vm_checkpoint_ok,
        detail: format!(
            "started={}, healthy={}, checkpoint_ok={}",
            vm_started, vm_healthy, vm_checkpoint_ok
        ),
    });

    let mut diag = Diagnostics::default();
    diag.check();
    stages.push(RuntimeStageReport {
        name: "diag",
        ok: diag.checks() >= 1,
        detail: format!("checks={}", diag.checks()),
    });

    let summary = SysRuntimeReport {
        platform_ready,
        block_inflight: blk.inflight(),
        scheduled_pid,
        getpid_value,
        io_completed,
        vm_started,
        diagnostics_checks: diag.checks(),
        vm_healthy,
        block_completed: block_completed + blk.completed(),
        io_cqes,
        vm_checkpoint_ok,
        sched_cycles,
    };

    SysRuntimeScenario { summary, stages }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_path_smoke() {
        let r = run_runtime_path();
        assert!(r.platform_ready);
        assert_eq!(r.block_inflight, 1);
        assert!(r.scheduled_pid == 1 || r.scheduled_pid == 2);
        assert_eq!(r.getpid_value, r.scheduled_pid as i64);
        assert!(r.io_completed);
        assert!(r.vm_started);
        assert_eq!(r.diagnostics_checks, 1);
        assert!(r.vm_healthy);
        assert!(r.block_completed >= 1);
        assert!(r.io_cqes >= 3);
        assert!(r.vm_checkpoint_ok);
        assert_eq!(r.sched_cycles, 3);
    }

    #[test]
    fn runtime_scenario_stages_all_ok() {
        let s = run_runtime_scenario();
        assert!(s.all_ok());
        assert!(s.stages.len() >= 8);
    }
}
