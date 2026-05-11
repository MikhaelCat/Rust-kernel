#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub name: &'static str,
}
impl Board {
    pub fn generic() -> Self {
        Self { name: "generic" }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn board_name() {
        assert_eq!(Board::generic().name, "generic");
    }
}
