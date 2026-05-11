#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerComponent06 {
    pub id: u32,
}
impl PowerComponent06 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(PowerComponent06::new(7).id, 7);
    }
}
