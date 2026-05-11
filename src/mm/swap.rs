#[derive(Debug, Default)]
pub struct SwapManager {
    pub swapped_pages: usize,
}
impl SwapManager {
    pub fn swap_out(&mut self) {
        self.swapped_pages += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swap_out() {
        let mut s = SwapManager::default();
        s.swap_out();
        assert_eq!(s.swapped_pages, 1);
    }
}
