#[derive(Debug, Default)]
pub struct GroStats {
    merged: u64,
}
impl GroStats {
    pub fn merge_one(&mut self) {
        self.merged += 1;
    }
    pub fn merged(&self) -> u64 {
        self.merged
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gro_merge() {
        let mut g = GroStats::default();
        g.merge_one();
        assert_eq!(g.merged(), 1);
    }
}
