#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityComponent10 {
    pub id: u32,
}
impl SecurityComponent10 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(SecurityComponent10::new(7).id, 7);
    }
}
