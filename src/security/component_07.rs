#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityComponent07 {
    pub id: u32,
}
impl SecurityComponent07 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(SecurityComponent07::new(7).id, 7);
    }
}
