#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Qdisc {
    FqCodel,
    PfifoFast,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn qdisc_variant() {
        assert_eq!(Qdisc::FqCodel as u8, 0);
    }
}
