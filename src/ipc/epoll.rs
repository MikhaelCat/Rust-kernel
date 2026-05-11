#[derive(Debug, Default)]
pub struct Epoll {
    watched: u64,
}
impl Epoll {
    pub fn add_fd(&mut self) {
        self.watched += 1;
    }
    pub fn watched(&self) -> u64 {
        self.watched
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_watch() {
        let mut e = Epoll::default();
        e.add_fd();
        assert_eq!(e.watched(), 1);
    }
}
