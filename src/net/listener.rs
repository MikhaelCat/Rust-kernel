#[derive(Debug, Default)]
pub struct Listener {
    backlog: usize,
    pending: usize,
}

impl Listener {
    pub fn listen(&mut self, backlog: usize) {
        self.backlog = backlog;
    }

    pub fn enqueue_conn(&mut self) -> bool {
        if self.pending < self.backlog {
            self.pending += 1;
            true
        } else {
            false
        }
    }

    pub fn accept(&mut self) -> bool {
        if self.pending > 0 {
            self.pending -= 1;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backlog_respected() {
        let mut l = Listener::default();
        l.listen(1);
        assert!(l.enqueue_conn());
        assert!(!l.enqueue_conn());
        assert!(l.accept());
    }
}
