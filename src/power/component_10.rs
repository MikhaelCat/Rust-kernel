#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerComponent10 {
    pub id: u32,
}
impl PowerComponent10 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(PowerComponent10::new(7).id, 7);
    }
}
