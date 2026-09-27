//! x86_64 Architecture Implementation for Linux Kernel

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub id: u32,
    pub cores_per_package: u32,
    pub threads_per_core: u32,
    pub features: CpuFeatures,
}

#[derive(Debug, Clone, Default)]
pub struct CpuFeatures {
    pub has_sse42: bool,
    pub has_avx: bool,
    pub has_nx: bool,
    pub has_pae: bool,
    pub has_lahf_lm: bool,
    pub has_lm: bool,
    pub has_clflush: bool,
    pub has_mmx: bool,
    pub has_mmx_ext: bool,
}

impl CpuInfo {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            cores_per_package: 1,
            threads_per_core: 1,
            features: CpuFeatures::default(),
        }
    }
}

#[derive(Debug)]
pub struct ArchSystem {
    pub cpus: HashMap<u32, CpuInfo>,
    pub paging_enabled: bool,
    pub smp_active: bool,
}

impl Default for ArchSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchSystem {
    pub fn new() -> Self {
        let mut arch = Self {
            cpus: HashMap::new(),
            paging_enabled: false,
            smp_active: false,
        };
        // Add CPU 0 by default
        arch.cpus.insert(0, CpuInfo::new(0));
        arch
    }

    pub fn add_cpu(&mut self, cpu_id: u32) {
        self.cpus.insert(cpu_id, CpuInfo::new(cpu_id));
    }

    pub fn bootstrap(&mut self) {
        self.smp_active = self.cpus.len() > 1;
        self.enable_paging();
    }

    pub fn enable_paging(&mut self) {
        // Enable long mode and paging
        unsafe {
            core::arch::asm!(
                "mov rax, cr0",
                "or rax, 1 << 31", // PE (Protection Enable)
                "mov cr0, rax",
                options(nostack)
            );
        }
        self.paging_enabled = true;
    }

    pub fn paging_enabled(&self) -> bool {
        self.paging_enabled
    }

    pub fn smp_active(&self) -> bool {
        self.smp_active
    }

    pub fn current_cpu(&self) -> Option<&CpuInfo> {
        self.cpus.get(&0)
    }
}
