#[derive(Debug, Default)]
pub struct Acpi {
    pub tables_loaded: bool,
    tables: Vec<&'static str>,
}

impl Acpi {
    pub fn load(&mut self) {
        self.tables_loaded = true;
        self.tables = vec!["DSDT", "FADT", "MADT"];
    }

    pub fn has_table(&self, name: &str) -> bool {
        self.tables.iter().any(|t| *t == name)
    }

    pub fn table_count(&self) -> usize {
        self.tables.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acpi_tables_inventory() {
        let mut a = Acpi::default();
        a.load();
        assert!(a.tables_loaded);
        assert!(a.has_table("MADT"));
        assert_eq!(a.table_count(), 3);
    }
}
