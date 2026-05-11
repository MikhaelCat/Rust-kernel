#[derive(Debug, Default)]
pub struct Topology {
    cpus: u32,
}
impl Topology {
    pub fn set_cpus(&mut self, n: u32) {
        self.cpus = n;
    }
    pub fn cpus(&self) -> u32 {
        self.cpus
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn set_topology() {
        let mut t = Topology::default();
        t.set_cpus(8);
        assert_eq!(t.cpus(), 8);
    }
}
