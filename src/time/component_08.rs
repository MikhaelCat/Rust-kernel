#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeComponent08 {
    pub id: u32,
}
impl TimeComponent08 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(TimeComponent08::new(7).id, 7);
    }
}
