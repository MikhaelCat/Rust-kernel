#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitrdImage {
    pub size: usize,
}
impl InitrdImage {
    pub fn new(size: usize) -> Self {
        Self { size }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initrd_new() {
        assert_eq!(InitrdImage::new(1024).size, 1024);
    }
}
