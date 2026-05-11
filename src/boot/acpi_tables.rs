#[derive(Debug, Default)]
pub struct AcpiTables {
    loaded: bool,
}
impl AcpiTables {
    pub fn load(&mut self) {
        self.loaded = true;
    }
    pub fn loaded(&self) -> bool {
        self.loaded
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_tables() {
        let mut a = AcpiTables::default();
        a.load();
        assert!(a.loaded());
    }
}
