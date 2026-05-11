#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeComponent07 {
    pub id: u32,
}
impl TimeComponent07 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(TimeComponent07::new(7).id, 7);
    }
}
