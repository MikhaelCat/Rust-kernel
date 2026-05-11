use super::{DriverSystem, iommu::Iommu, irqchip::IrqChip, probe::probe_device};

pub fn bringup() -> bool {
    let mut d = DriverSystem::default();
    d.load_base();

    let mut iommu = Iommu::default();
    iommu.map();

    let mut irq = IrqChip::default();
    irq.route();

    d.has("uart")
        && probe_device(0x8086, 0x100e).matched
        && iommu.mappings() == 1
        && irq.routed() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bringup_ok() {
        assert!(bringup());
    }
}
