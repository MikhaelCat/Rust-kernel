#[derive(Debug, Default)]
pub struct BufferCache {
    hits: u64,
}
impl BufferCache {
    pub fn hit(&mut self) {
        self.hits += 1;
    }
    pub fn hits(&self) -> u64 {
        self.hits
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cache_hit() {
        let mut c = BufferCache::default();
        c.hit();
        assert_eq!(c.hits(), 1);
    }
}
