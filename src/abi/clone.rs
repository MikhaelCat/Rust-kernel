#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CloneFlags {
    pub share_vm: bool,
    pub share_files: bool,
}

impl CloneFlags {
    pub fn from_bits(bits: u64) -> Self {
        Self {
            share_vm: (bits & 0x100) != 0,
            share_files: (bits & 0x400) != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bits() {
        let f = CloneFlags::from_bits(0x500);
        assert!(f.share_vm);
        assert!(f.share_files);
    }
}
