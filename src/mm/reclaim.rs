#[derive(Debug, Default)]
pub struct Reclaimer {
    scanned: u64,
}
impl Reclaimer {
    pub fn scan(&mut self) {
        self.scanned += 1;
    }
    pub fn scanned(&self) -> u64 {
        self.scanned
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reclaim_scan() {
        let mut r = Reclaimer::default();
        r.scan();
        assert_eq!(r.scanned(), 1);
    }
}
