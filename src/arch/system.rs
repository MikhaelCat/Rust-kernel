use super::{cache::CpuCache, microcode::Microcode, numa::Numa, paging::ArchPaging, tlb::Tlb};

#[derive(Debug, Default)]
pub struct ArchSystem {
    paging: ArchPaging,
    tlb: Tlb,
    cache: CpuCache,
    numa: Numa,
    microcode: Microcode,
}

impl ArchSystem {
    pub fn bootstrap(&mut self) {
        self.paging.enable();
        self.tlb.flush_all();
        self.cache.invalidate_all();
        self.numa.set_nodes(1);
        self.microcode.load(1);
    }

    pub fn paging_enabled(&self) -> bool {
        self.paging.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_bootstrap_flow() {
        let mut a = ArchSystem::default();
        a.bootstrap();
        assert!(a.paging_enabled());
    }
}
