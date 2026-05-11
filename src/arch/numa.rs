#[derive(Debug, Default)]
pub struct Numa {
    nodes: u32,
}
impl Numa {
    pub fn set_nodes(&mut self, n: u32) {
        self.nodes = n;
    }
    pub fn nodes(&self) -> u32 {
        self.nodes
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_nodes() {
        let mut n = Numa::default();
        n.set_nodes(2);
        assert_eq!(n.nodes(), 2);
    }
}
