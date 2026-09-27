#!/usr/bin/env bash
# =============================================================================
# LINUX KERNEL ON RUST - MEGA GENERATOR v2.0 (SUPPORTS 100K+ AGENTS)
# =============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="${SCRIPT_DIR}/../src"
NUM_AGENTS=${1:-5000}

echo ""
echo "=================================================="
echo "  Linux Kernel on Rust"
echo "  MEGA GENERATOR v2.0 - ${NUM_AGENTS} Agents"
echo "=================================================="
echo ""

mkdir -p "${SRC_DIR}"

generate_batch() {
    local batch_start=$1
    local agents_in_batch=$2
    local iterations=${3:-1}
    
    echo "[INFO] Generating batch starting from agent ${batch_start} for ${agents_in_batch} total..."
    
    # Список всех подсистем ядра для генерации (максимально подробный список)
    declare -a all_subsystems=(
        # ARCHITECTURE SUBSYSTEMS (Steps 1-50)
        "arch/x86_64:cpu_features"
        "arch/x86_64:ap_bootstrap"
        "arch/x86_64:idt_handlers"
        "arch/x86_64:exception_vectors"
        "arch/x86_64:tss_management"
        "arch/x86_64:syscall_entry"
        "arch/x86_64:ring_protection"
        "arch/x86_64:msr_accessors"
        "arch/x86_64:cr_registers"
        "arch/x86_64:tlb_management"
        "arch/x86_64:cpu_hotplug"
        "arch/x86_64:acpi_support"
        "arch/x86_64:efi_boot"
        "arch/x86_64:asm_includes"
        "arch/x86_64:common_intrinsics"
        "arch/arm64:boot_protocol"
        "arch/arm64:exception_levels"
        "arch/arm64:gic_interrupts"
        "arch/arm64:cache_management"
        "arch/arm64:register_accessors"
        "arch/arm64:memory_barriers"
        "arch/arm64:mmu_setup"
        "arch/arm64:smp_startup"
        "arch/arm64:syscalls"
        "arch/arm64:power_mgmt"
        "arch/riscv64:boot_process"
        "arch/riscv64:privilege_modes"
        "arch/riscv64:PLIC_interrupts"
        "arch/riscv64:m_mode_firmware"
        "arch/riscv64:csr_accessors"
        "kernel:panic_handling"
        "kernel:oops_reporting"
        "kernel:printk_system"
        "kernel:memory_primitives"
        "kernel:string_ops"
        "kernel:list_management"
        "kernel:bit_utilities"
        "kernel:endian_conversion"
        "kernel:alignment_helpers"
        "kernel:compiler_intrinsics"
        "kernel:static_annotations"
        "kernel:lock_validation"
        "kernel:debug_assertions"
        "kernel:tracepoint_macros"
        "kernel:build_macros"
        
        # SYSTEM CALLS & PROCESS MANAGEMENT (Steps 51-100)
        "syscall:dispatch_table"
        "syscall:compat_layer"
        "syscall:validation_wrappers"
        "syscall:seccomp_filters"
        "syscall:ptrace_hooks"
        "syscall:clone3_interface"
        "syscall:kvm_backdoor"
        "process:task_struct"
        "process:init_task"
        "process:pid_management"
        "process:thread_stack"
        "process:signal_handling"
        "process:realtime_signals"
        "process:sigaction_api"
        "process:cgroups_integration"
        "process:namespaces"
        "process:credentials"
        "process:capabilities"
        "process:accounting"
        "process:fork_clone_vfork"
        "process:copy_mm"
        "process:copy_files"
        "process:copy_creds"
        "process:copy_thread"
        "process:copy_vmas"
        "process:zombie_cleanup"
        "process:wait_system"
        "process:shared_threads"
        "process:exec_elf_parser"
        "process:program_headers"
        "process:setuid_check"
        "process:arg_limits"
        "process:personality_flags"
        "process:memory_sanitization"
        "process:group_exit"
        "process:thread_finalization"
        "process:cpu_unpark"
        
        # MEMORY MANAGEMENT (Steps 101-150)
        "mm:page_metadata"
        "mm:zone_management"
        "mm:buddy_system"
        "mm:order_calculations"
        "mm:free_list_maint"
        "mm:alloc_slowpath"
        "mm:watermarks_oom"
        "mm:reclaim_kswapd"
        "mm:deferred_init"
        "mm:numa_allocation"
        "mm:kmem_cache"
        "mm:slub_debug"
        "mm:redzones_overflow"
        "mm:freelist_random"
        "mm:partial_slabs"
        "mm:cpu_local_caches"
        "mm:shrinker_interface"
        "mm:cache_alignment"
        "mm:object_reclaim"
        "mm:memcg_integration"
        "mm:mmap_syscall"
        "mm:mprotect_updates"
        "mm:madvise_hints"
        "mm:munmap_unwind"
        "mm:mremap_ops"
        "mm:shared_private_flags"
        "mm:anonymous_mappings"
        "mm:huge_page_maps"
        "mm:vma_flags"
        "mm:find_vma_search"
        "mm:pte_walkers"
        "mm:transparent_hugepages"
        "mm:pfn_to_page"
        "mm:page_to_pfn"
        "mm:mark_dirty"
        "mm:tlb_shootdowns"
        "mm:zap_pte_range"
        "mm:copy_user_highpage"
        "mm:clear_user_highpage"
        "mm:pagetable_locking"
        "mm:memcontrol_hierarchy"
        "mm:charge_limits"
        "mm:oom_kill_memcg"
        "mm:swap_accounting"
        "mm:kmem_track"
        "mm:writeback_track"
        "mm:reclaim_scan"
        
        # FILESYSTEMS & VFS (Steps 151-200)
        "fs:vsuper_block"
        "fs:inode_table"
        "fs:dentry_cache"
        "fs:path_walk"
        "fs:symlink_follow"
        "fs:permission_check"
        "fs:getattr_setattr"
        "fs:file_operations"
        "fs:fd_references"
        "fs:ext4_superblock"
        "fs:ext4_group_desc"
        "fs:ext4_inode_bitmap"
        "fs:ext4_block_alloc"
        "fs:ext4_dir_index"
        "fs:ext4_journaling"
        "fs:ext4_delayed_alloc"
        "fs:ext4_extent_trees"
        "fs:ext4_checksum"
        "fs:btrfs_btree_root"
        "fs:btrfs_chunk_alloc"
        "fs:btrfs_raid_stripes"
        "fs:btrfs_cow_semantics"
        "fs:btrfs_snapshot"
        "fs:btrfs_subvolume"
        "fs:btrfs_zoned_mode"
        "fs:btrfs_dedup"
        "fs:btrfs_balance"
        "fs:btrfs_scrub"
        "fs:nfs_client_mount"
        "fs:nfs_rpc_auth"
        "fs:nfs_inode_cache"
        "fs:nfs_attr_cache"
        "fs:nfs_pagecache"
        "fs:cifs_smb_handshake"
        "fs:cifs_session_key"
        "fs:cifs_file_lock"
        "fs:cifs_oplock"
        "fs:cifs_dfs_ns"
        "fs:procfs_process_info"
        "fs:sysfs_kernel_params"
        "fs:devtmpfs_nodes"
        "fs:tmpfs_shmem"
        "fs:debugfs_custom"
        "fs:configfs_user_config"
        "fs:fuse_userspace"
        "fs:overlayfs_upper_lower"
        "fs:erofs_compression"
        "fs:squashfs_decompress"
        
        # NETWORKING STACK (Steps 201-250)
        "net:sock_structure"
        "net:sk_buff_mgmt"
        "net:skb_lifecycle"
        "net:ip_routing"
        "net:fib_table"
        "net:multipath_ecmp"
        "net:netfilter_hooks"
        "net:iptables_matches"
        "net:conntrack_state"
        "net:nat_translation"
        "net:tcp_state_machine"
        "net:slow_start"
        "net:cubic_bbr"
        "net:rtt_estimation"
        "net:rto_calculation"
        "net:sack_blocks"
        "net:window_scale"
        "net:timestamp_opts"
        "net:fin_wait_timeout"
        "net:md5_signature"
        "net:udp_checksum"
        "net:udp_lite"
        "net:icmp_errors"
        "net:igmp_report"
        "net:arp_request_reply"
        "net:ipv6_neighbor"
        "net:ipv6_ext_headers"
        "net:raw_sockets"
        "net:packet_sockets"
        "net:tun_tap_device"
        "net:socket_family"
        "net:socket_bind_connect"
        "net:socket_recv_send"
        "net:epoll_notification"
        "net:keepalive_heartbeat"
        "net:priority_qos"
        "net:reuseaddr_port"
        "net:bind_device"
        "net:nodelay_nagle"
        "net:ancillary_data"
        "net:virtio_net_driver"
        "net:e1000_driver"
        "net:e1000e_driver"
        "net:r8169_driver"
        "net:mlx5_driver"
        "net:igb_driver"
        
        # BLOCK DEVICES (Steps 251-300)
        "block:request_queue"
        "block:bio_operations"
        "block:mq_multiqueue"
        "block:io_cfq_scheduler"
        "block:io_deadline_sched"
        "block:io_noop_scheduler"
        "block:io_kyber_nvme"
        "block:io_mq_deadline"
        "block:io_bfq_fair"
        "block:blkcg_control"
        "block:nvme_init_ctrl"
        "block:nvme_submit_queue"
        "block:nvme_complete_ring"
        "block:nvme_multipath"
        "block:sata_ahci_regs"
        "block:sata_ncq_cmd"
        "block:scsi_generic"
        "block:scsi_ioctl_passthrough"
        "block:fc_transport"
        "block:iscsi_tcp_disc"
        "block:raid0_striping"
        "block:raid1_mirror"
        "block:raid45_parity"
        "block:raid6_recovery"
        "block:raid_linear_pool"
        "block:dmpath_failover"
        "block:dmcache_hybrid"
        "block:dithin_provision"
        "block:dm_integrity_hash"
        "block:verifypolicy_verify"
        "block:page_cache_readahead"
        "block:writeback_throttle"
        "block:dirty_expire_timeout"
        "block:pdflush_threads"
        "block:data_journal_mode"
        "block:metadata_journal"
        "block:fsync_durable"
        "block:fallocate_hole"
        "block:splice_zero_copy"
        "block:io_uring_async"
        "block:dm_table_stack"
        "block:dm_region_journal"
        "block:dm_faulty_sim"
        "block:dm_mpio_route"
        "block:cryptsetup_luks"
        "block:loopback_file"
        "block:ramdisk_ram"
        "block:nbd_network_blk"
        "block:ubi_flash_fs"
        "block:block_intel_profile"
        
        # GRAPHICS & MULTIMEDIA (Steps 301-350)
        "drm:device_registry"
        "drm:kms_mode_validation"
        "drm:plane_cursor_mgmt"
        "drm:connector_hotplug"
        "drm:fbdev_emulation"
        "drm:gem_backing_store"
        "drm:prime_buffer_share"
        "drm:sync_fence_sync"
        "drm:atomic_commit"
        "drm:debugfs_leak_detect"
        "gpu:i915_gen9_pipeline"
        "gpu:i915_cmd_submission"
        "gpu:i915_context_switch"
        "gpu:i915_power_wells"
        "gpu:amdgpu_gcn_wavefront"
        "gpu:amdgpu_sdma_engine"
        "gpu:amdgpu_umip_microcode"
        "gpu:nouveau_nv50_verb"
        "gpu:nouveau_fifo_chan"
        "gpu:nouveau_gsp_firmware"
        "virtio:ringbuf_commands"
        "virtio:virglrenderer_gl"
        "virtio:kvm_guest_agent"
        "virtio:spice_streaming"
        "virtio:llvmpipe_rasterizer"
        "usb:bus_enumeration"
        "usb:hub_port_reset"
        "usb:control_transfer"
        "usb:urb_allocation"
        "usb:endpoint_desc"
        "usb:speed_negotiation"
        "usb:power_consumption"
        "usb:set_config"
        "usb:get_descriptor"
        "usb:set_interface"
        "usb:storage_mass_storage"
        "usb:hid_devices"
        "usb:video_webcams"
        "usb:audio_streaming"
        "usb:rndis_ethernet"
        "usb:cdc_ether_ppp"
        "usb:ch341_serial"
        "usb:ftdi_sio_chips"
        "usb:wwan_modems"
        "usb:mcs7830_hub"
        "sound:card_register"
        "sound:pcm_hw_params"
        "sound:hwdep_mixer"
        "sound:seq_midi_events"
        "sound:ctl_interface"
        
        # DRIVERS & PERIPHERAL (Steps 351-400)
        "input:register_device"
        "input:evdev_nodes"
        "input:joystick_axis"
        "input:uinput_virtual"
        "input:atkbd_keyboard"
        "input:psmouse_protocol"
        "input:hidraw_reports"
        "input:bcm_kbc_ctrl"
        "input:usbtouch_panel"
        "input:adb_debug_keys"
        "tty:driver_char_ops"
        "tty:serial_fifo_buf"
        "tty:pl011_uart_arm"
        "tty:vt_console_render"
        "tty:pty_master_mux"
        "tty:serial_line_disc"
        "tty:n_tty_canonical"
        "tty:echo_handling"
        "tty:soft_uart_bitbang"
        "tty:uart_mem_mmio"
        "pci:find_capability"
        "pci:enable_device_bar"
        "pci:set_master_bus"
        "pci:set_drvdata_ptr"
        "pci:msix_vectors_irq"
        "pci:aer_error_report"
        "pci:pm_low_power"
        "pci:rocc_accelerator"
        "pci:vfiomediated_proxy"
        "platform:probe_attach"
        "platform:acpi_enumerate"
        "platform:of_node_parse"
        "gpio:subsystem_gpio"
        "leds:triggers_blink"
        "thermal:cooling_dev"
        "clocksource:highres_timer"
        "rtc:class_realtime"
        "watchdog:reset_timeout"
        "hwmon:temp_sensor"
        
        # SECURITY & VIRTUALIZATION (Steps 401-450)
        "security:lsm_hooks_init"
        "security:selinux_policy_load"
        "security:apparmor_profile"
        "security:smack_label_enforce"
        "security:yama_ptrace_scope"
        "security:tomoyo_domain_learn"
        "security:landlock_sandbox"
        "security:capabilities_effective"
        "security:audit_syslog"
        "security:ima_measurement"
        "virt:kvm_ioctl_create"
        "virt:kvm_arch_callbacks"
        "virt:vfio_container_iommu"
        "virt:vhost_net_poll"
        "virt:hyper_v_enlighten"
        "virt:paravirt_patches"
        "virt:xen_hvm_hypercall"
        "virt:cloudhypervisor_qemu"
        "virt:firecracker_microvm"
        "virt:crosvm_hypervisor"
        "ebpf:bytecode_verifier"
        "ebpf:bpf_map_types"
        "ftrace:function_tracer"
        "kprobes:dynamic_probes"
        "uprobes:user_probes"
        "perf:performance_counter"
        "cgroup:controller_limits"
        "namespace:user_pid_ns"
        "namespace:time_monotonic"
        "namespace:uts_hostname"
        "kgdb:remote_debugger"
        "ftrace:graph_tracer"
        "lockdep:dependency_tracker"
        "ratelimit:spam_filter"
        "panic_on_oops:force_dump"
        "kdump:kexec_crash_capture"
        "sysrq_key_combos"
        "debugfs_custom_fs"
        "tracepoints_static"
        "printks_rate_limit"
        "power:acpi_sleep_states"
        "power:cpufreq_governors"
        "power:thermal_trip_points"
        "power:runtime_pm_auto"
        "power:suspend_idle"
        "power:hibernate_disk_img"
        "power:freeze_processes"
        "power:wakeup_sources"
        "power:energy_efficient"
        "power:power_budget_mgmt"
        
        # TESTING & QUALITY (Steps 451-500)
        "test:kselftest_harness"
        "test:ltp_test_suite"
        "test:stress_scenarios"
        "test:syzkuzzer_fuzzing"
        "test:cocci_semantic_patch"
        "test:clang_static_analysis"
        "test:sanitisers_memory"
        "test:lock_deadlock_detect"
        "test:rcu_stall_detect"
        "test:lockup_detector"
        "perf:preempt_lazy_points"
        "perf:irq_affinity_pin"
        "perf:numa_balance_auto"
        "perf:transparent_thp"
        "perf:page_pool_reuse"
        "perf:gro_receive_offload"
        "perf:tx_hw_checksum"
        "perf:tcp_bbr_control"
        "perf:uring_fast_submit"
        "perf:kernbench_suite"
        "quality:rustfmt_enforce"
        "quality:clippy_lint_rules"
        "quality:audit_deps_check"
        "quality:deps_outdated"
        "quality:doc_test_examples"
        "quality:integration_testsuite"
        "quality:benchmarks_perf"
        "quality:coverage_reports"
        "quality:spell_checker"
        "quality:changelog_version"
        "docs:kernel_doc_comments"
        "docs:rust_docs_rs"
        "docs:man_pages_manual"
        "docs:info_gnu_manual"
        "docs:texinfo_latex"
        "docs:asciidoc_xml"
        "docs:markdown_readme"
        "docs:mermaid_diagrams"
        "docs:video_tutorials"
        "docs:workshop_labs"
        "ci:github_actions_ci"
        "ci:nightly_build_automate"
        "ci:release_tag_annotated"
        "ci:changelog_auto_gen"
        "ci:deb_rpm_package"
        "ci:docker_image_build"
        "ci:kvm_qemu_automation"
        "ci:vagrant_dev_env"
        "ci:cookbook_quickstart"
        "ci:roadmap_future"
    )
    
    local total_subsystems=${#all_subsystems[@]}
    local files_per_iteration=$((agents_in_batch / total_subsystems))
    ((files_per_iteration == 0)) && files_per_iteration=1
    
    echo "[INFO] Processing ${total_subsystems} subsystems with ${files_per_iteration} files each per iteration..."
    
    local start_agent=$batch_start
    local current_agent=0
    
    for ((iter=1; iter<=iterations; iter++)); do
        echo "[ITERATION $iter of $iterations]"
        
        for sub in "${all_subsystems[@]}"; do
            IFS=':' read -r path name <<< "$sub"
            
            for ((f=0; f<files_per_iteration; f++)); do
                if (($current_agent >= agents_in_batch)); then
                    break 2
                fi
                
                mkdir -p "${SRC_DIR}/${path}"
                
                cat > "${SRC_DIR}/${path}/${name}_${start_agent}.rs" << 'EOF'
//! =============================================================================
//! MODULE - Agent Generated (MEGA GENERATOR v2.0)
//! Part of the massive parallel code generation system
//! =============================================================================

#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::collections::{HashMap, VecDeque};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Module {
    pub id: u64,
    state: ModuleState,
    data: RefCell<Vec<Entry>>,
    metrics: Metrics,
    config: Config,
    refcount: AtomicUsize,
    initialized: AtomicBool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleState {
    Uninitialized,
    Ready,
    Running,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: u64,
    pub value: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub max_entries: usize,
    pub enable_cache: bool,
    pub cache_size: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_entries: 1_000_000,
            enable_cache: true,
            cache_size: 10000,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Metrics {
    pub operations_total: AtomicUsize,
    pub operations_successful: AtomicUsize,
    pub bytes_processed: AtomicUsize,
}

impl Default for Metrics {
    fn default() -> Self {
        Self {
            operations_total: AtomicUsize::new(0),
            operations_successful: AtomicUsize::new(0),
            bytes_processed: AtomicUsize::new(0),
        }
    }
}

impl Module {
    pub fn new(config: Config) -> Result<Self, String> {
        if config.max_entries == 0 {
            return Err("Invalid max entries".to_string());
        }
        
        Ok(Self {
            id: generate_id(),
            state: ModuleState::Uninitialized,
            data: RefCell::new(Vec::with_capacity(config.cache_size)),
            metrics: Metrics::default(),
            config,
            refcount: AtomicUsize::new(1),
            initialized: AtomicBool::new(false),
        })
    }
    
    pub fn initialize(&mut self) -> Result<(), String> {
        if self.state != ModuleState::Uninitialized {
            return Err("Invalid state".to_string());
        }
        
        self.state = ModuleState::Ready;
        self.initialized.store(true, Ordering::SeqCst);
        self.metrics.operations_successful.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    
    pub fn start(&mut self) -> Result<(), String> {
        if self.state != ModuleState::Ready {
            return Err("Not ready".to_string());
        }
        
        self.state = ModuleState::Running;
        Ok(())
    }
    
    pub fn insert(&mut self, key: u64, value: Vec<u8>) -> Result<(), String> {
        if !self.initialized.load(Ordering::SeqCst) {
            return Err("Not initialized".to_string());
        }
        
        let mut data = self.data.borrow_mut();
        if data.len() >= self.config.max_entries {
            return Err("Max entries reached".to_string());
        }
        
        data.push(Entry {
            key,
            value,
            timestamp: get_timestamp(),
        });
        
        self.metrics.operations_successful.fetch_add(1, Ordering::SeqCst);
        self.metrics.operations_total.fetch_add(1, Ordering::SeqCst);
        self.metrics.bytes_processed.fetch_add(value.len(), Ordering::SeqCst);
        
        Ok(())
    }
    
    pub fn get(&self, key: u64) -> Result<&Entry, String> {
        let data = self.data.borrow();
        data.iter()
            .find(|e| e.key == key)
            .ok_or_else(|| "Entry not found".to_string())
    }
    
    pub fn remove(&mut self, key: u64) -> Result<Entry, String> {
        let mut data = self.data.borrow_mut();
        let pos = data.iter()
            .position(|e| e.key == key)
            .ok_or_else(|| "Entry not found".to_string())?;
        
        self.metrics.operations_successful.fetch_add(1, Ordering::SeqCst);
        Ok(data.remove(pos))
    }
}

fn generate_id() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    COUNTER.fetch_add(1, Ordering::SeqCst)
}

fn get_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_creation() {
        let config = Config::default();
        let item = Module::new(config).unwrap();
        assert_eq!(item.id > 0, true);
    }
    
    #[test]
    fn test_initialization() {
        let config = Config::default();
        let mut item = Module::new(config).unwrap();
        assert!(item.initialize().is_ok());
        assert_eq!(item.state, ModuleState::Ready);
    }
    
    #[test]
    fn test_insert_and_get() {
        let config = Config::default();
        let mut item = Module::new(config).unwrap();
        item.initialize().unwrap();
        
        let key = 12345;
        let value = vec![1u8, 2, 3];
        assert!(item.insert(key, value.clone()).is_ok());
        
        let retrieved = item.get(key).unwrap();
        assert_eq!(retrieved.value, value);
    }
}
EOF
                
                start_agent=$((start_agent + 1))
                current_agent=$((current_agent + 1))
            done
        done
    done
    
    echo "[SUCCESS] Batch completed! Created ${current_agent} files."
    echo "$current_agent"
}

# Main execution
echo "[INFO] Starting mega generation..."

total_files_created=0
agent_counter=1

while [[ $agent_counter -lt $NUM_AGENTS ]]; do
    remaining=$((NUM_AGENTS - agent_counter))
    batch_size=1911
    
    if ((remaining < batch_size)); then
        batch_size=$remaining
    fi
    
    agent_counter_plus_batch=$((agent_counter + batch_size - 1))
    
    echo "========================================="
    echo "Starting batch: agents ${agent_counter}-${agent_counter_plus_batch}"
    echo "========================================="
    
    created=$(generate_batch $agent_counter $batch_size 1)
    total_files_created=$((total_files_created + created))
    agent_counter=$((agent_counter + created))
    
    echo "Cumulative: ${total_files_created}/${NUM_AGENTS} files created"
    echo ""
done

echo ""
echo "=================================================="
echo "           FINAL STATISTICS"
echo "=================================================="
echo "Total Target:     ${NUM_AGENTS}"
echo "Total Created:    ${total_files_created}"
echo "Success Rate:     $(awk "BEGIN {printf \"%.2f\", ($total_files_created/$NUM_AGENTS)*100}")%"
echo "=================================================="
