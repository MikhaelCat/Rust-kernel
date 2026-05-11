#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerComponent09 {
    pub id: u32,
}
impl PowerComponent09 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(PowerComponent09::new(7).id, 7);
    }
}
