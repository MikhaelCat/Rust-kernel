#[derive(Debug, Default)]
pub struct ArchPaging {
    pub enabled: bool,
}
impl ArchPaging {
    pub fn enable(&mut self) {
        self.enabled = true;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enable_paging() {
        let mut p = ArchPaging::default();
        p.enable();
        assert!(p.enabled);
    }
}
