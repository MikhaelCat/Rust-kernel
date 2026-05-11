#[derive(Debug, Default)]
pub struct Vmalloc {
    allocs: usize,
}
impl Vmalloc {
    pub fn alloc(&mut self) -> usize {
        self.allocs += 1;
        self.allocs
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn alloc_id() {
        let mut v = Vmalloc::default();
        assert_eq!(v.alloc(), 1);
    }
}
