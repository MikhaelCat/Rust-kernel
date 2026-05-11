#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DtbBlob {
    pub size: usize,
}
impl DtbBlob {
    pub fn parse(bytes: &[u8]) -> Self {
        Self { size: bytes.len() }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dtb_parse() {
        assert_eq!(DtbBlob::parse(&[1, 2, 3]).size, 3);
    }
}
