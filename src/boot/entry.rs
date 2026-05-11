#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryPoint(pub u64);

impl EntryPoint {
    pub fn is_null(self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_non_null_check() {
        assert!(!EntryPoint(0x1000).is_null());
        assert!(EntryPoint(0).is_null());
    }
}
