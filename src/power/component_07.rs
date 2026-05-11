#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowerComponent07 {
    pub id: u32,
}
impl PowerComponent07 {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create() {
        assert_eq!(PowerComponent07::new(7).id, 7);
    }
}
