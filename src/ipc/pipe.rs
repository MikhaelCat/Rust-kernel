use std::collections::VecDeque;
#[derive(Debug, Default)]
pub struct Pipe {
    q: VecDeque<u8>,
}
impl Pipe {
    pub fn write(&mut self, b: u8) {
        self.q.push_back(b);
    }
    pub fn read(&mut self) -> Option<u8> {
        self.q.pop_front()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pipe_fifo() {
        let mut p = Pipe::default();
        p.write(1);
        p.write(2);
        assert_eq!(p.read(), Some(1));
    }
}
