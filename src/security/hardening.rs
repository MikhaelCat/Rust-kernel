#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HardeningProfile {
    pub canaries: bool,
    pub cfi: bool,
}

impl HardeningProfile {
    pub fn baseline() -> Self {
        Self {
            canaries: true,
            cfi: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_enabled() {
        let p = HardeningProfile::baseline();
        assert!(p.canaries && p.cfi);
    }
}
