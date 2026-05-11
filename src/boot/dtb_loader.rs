#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DtbLoader {
    pub loaded: bool,
}
impl DtbLoader {
    pub fn load(blob: &[u8]) -> Self {
        Self {
            loaded: !blob.is_empty(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dtb_loaded() {
        assert!(DtbLoader::load(&[1, 2]).loaded);
    }
}
