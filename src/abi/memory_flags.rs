#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmapProt {
    pub r: bool,
    pub w: bool,
    pub x: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmapFlags {
    pub private_map: bool,
    pub anonymous: bool,
}

impl MmapProt {
    pub fn from_bits(bits: u64) -> Self {
        Self {
            r: (bits & 0x1) != 0,
            w: (bits & 0x2) != 0,
            x: (bits & 0x4) != 0,
        }
    }
}

impl MmapFlags {
    pub fn from_bits(bits: u64) -> Self {
        Self {
            private_map: (bits & 0x02) != 0,
            anonymous: (bits & 0x20) != 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mmap_bits() {
        let p = MmapProt::from_bits(0x3);
        assert!(p.r && p.w && !p.x);
        let f = MmapFlags::from_bits(0x22);
        assert!(f.private_map && f.anonymous);
    }
}
