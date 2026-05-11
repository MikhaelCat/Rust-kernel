#[derive(Debug, Default)]
pub struct Cma {
    reserved: usize,
}
impl Cma {
    pub fn reserve(&mut self, bytes: usize) {
        self.reserved += bytes;
    }
    pub fn reserved(&self) -> usize {
        self.reserved
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reserve_cma() {
        let mut c = Cma::default();
        c.reserve(4096);
        assert_eq!(c.reserved(), 4096);
    }
}
