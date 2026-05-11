#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Icmpv6Echo {
    pub id: u16,
    pub seq: u16,
}
impl Icmpv6Echo {
    pub fn new(id: u16, seq: u16) -> Self {
        Self { id, seq }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn echo_new() {
        assert_eq!(Icmpv6Echo::new(1, 2).seq, 2);
    }
}
