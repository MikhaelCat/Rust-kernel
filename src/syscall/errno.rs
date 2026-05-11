pub const EPERM: i32 = 1;
pub const ENOENT: i32 = 2;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constants() {
        assert_eq!(EPERM, 1);
        assert_eq!(ENOENT, 2);
    }
}
