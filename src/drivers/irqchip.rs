#[derive(Debug, Default)]
pub struct IrqChip {
    routed: u64,
}

impl IrqChip {
    pub fn route(&mut self) {
        self.routed += 1;
    }

    pub fn routed(&self) -> u64 {
        self.routed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_irq() {
        let mut c = IrqChip::default();
        c.route();
        assert_eq!(c.routed(), 1);
    }
}
