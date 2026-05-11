use super::listener::Listener;
use super::session::{NetSession, SessionState};

pub fn run_conn_matrix() -> bool {
    let mut l = Listener::default();
    l.listen(2);
    let e1 = l.enqueue_conn();
    let e2 = l.enqueue_conn();
    let e3 = l.enqueue_conn();
    let a1 = l.accept();

    let mut s = NetSession::new();
    s.establish();
    s.close();

    e1 && e2 && !e3 && a1 && s.state == SessionState::Closed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conn_matrix_ok() {
        assert!(run_conn_matrix());
    }
}
