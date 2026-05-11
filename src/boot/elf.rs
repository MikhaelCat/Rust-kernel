use super::error::BootError;

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElfImage {
    pub entry: u64,
    pub bytes: Vec<u8>,
}

impl ElfImage {
    pub fn parse(bytes: &[u8]) -> Result<Self, BootError> {
        if bytes.len() < 16 || bytes[0..4] != ELF_MAGIC {
            return Err(BootError::InvalidElfImage);
        }

        // Simplified entry extraction for deterministic tests.
        let entry = if bytes.len() >= 32 {
            u64::from_le_bytes([
                bytes[24], bytes[25], bytes[26], bytes[27], bytes[28], bytes[29], bytes[30],
                bytes[31],
            ])
        } else {
            0
        };

        Ok(Self {
            entry,
            bytes: bytes.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_elf() {
        let mut img = vec![0u8; 64];
        img[0..4].copy_from_slice(&ELF_MAGIC);
        img[24..32].copy_from_slice(&0x401000u64.to_le_bytes());
        let parsed = ElfImage::parse(&img).expect("parse failed");
        assert_eq!(parsed.entry, 0x401000);
    }

    #[test]
    fn reject_invalid_elf() {
        let img = vec![0u8; 64];
        assert_eq!(ElfImage::parse(&img), Err(BootError::InvalidElfImage));
    }
}
