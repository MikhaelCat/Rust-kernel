pub fn clamp_len(len: usize, max: usize) -> usize {
    if len > max { max } else { len }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamps() {
        assert_eq!(clamp_len(10, 8), 8);
        assert_eq!(clamp_len(4, 8), 4);
    }
}
