#[derive(Debug, Default)]
pub struct Tick {
    jiffies: u64,
}
impl Tick {
    pub fn inc(&mut self) {
        self.jiffies += 1;
    }
    pub fn jiffies(&self) -> u64 {
        self.jiffies
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tick_inc() {
        let mut t = Tick::default();
        t.inc();
        assert_eq!(t.jiffies(), 1);
    }
}
