#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeComponent05 {
    pub id: u32,
}
impl TimeComponent05 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(TimeComponent05::new(7).id, 7);
    }
}
