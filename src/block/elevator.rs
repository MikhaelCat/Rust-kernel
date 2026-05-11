#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Elevator {
    None,
    Deadline,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn elevator_variant() {
        assert_eq!(Elevator::Deadline as u8, 1);
    }
}
