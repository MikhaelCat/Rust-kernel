#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Term,
    Kill,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signal_variant() {
        assert_eq!(Signal::Term as u8, 0);
    }
}
