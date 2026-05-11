#[derive(Debug, Default)]
pub struct Ksm {
    merged_pages: u64,
}
impl Ksm {
    pub fn merge(&mut self) {
        self.merged_pages += 1;
    }
    pub fn merged(&self) -> u64 {
        self.merged_pages
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn merge_page() {
        let mut k = Ksm::default();
        k.merge();
        assert_eq!(k.merged(), 1);
    }
}
