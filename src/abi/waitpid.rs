pub fn waitpid_like(last_exited: Option<i64>, wanted: i64) -> i64 {
    match last_exited {
        Some(pid) if wanted == -1 || wanted == pid => pid,
        _ => -10, // ECHILD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn waitpid_matching() {
        assert_eq!(waitpid_like(Some(4), -1), 4);
        assert_eq!(waitpid_like(Some(4), 4), 4);
        assert_eq!(waitpid_like(Some(4), 3), -10);
    }
}
