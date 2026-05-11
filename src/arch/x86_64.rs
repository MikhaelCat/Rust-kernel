#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuFeatures {
    pub smep: bool,
    pub smap: bool,
    pub nx: bool,
}

impl CpuFeatures {
    pub fn baseline() -> Self {
        Self {
            smep: true,
            smap: true,
            nx: true,
        }
    }

    pub fn hardened(&self) -> bool {
        self.smep && self.smap && self.nx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_is_hardened() {
        let f = CpuFeatures::baseline();
        assert!(f.hardened());
    }
}
