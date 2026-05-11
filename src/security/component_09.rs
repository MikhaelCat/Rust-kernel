#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityComponent09 {
    pub id: u32,
}
impl SecurityComponent09 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(SecurityComponent09::new(7).id, 7);
    }
}
