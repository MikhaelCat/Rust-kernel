#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerComponent08 {
    pub id: u32,
}
impl PowerComponent08 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(PowerComponent08::new(7).id, 7);
    }
}
