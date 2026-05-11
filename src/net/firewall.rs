#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Accept,
    Drop,
}
pub fn inspect_port(dst_port: u16) -> Verdict {
    if dst_port == 23 {
        Verdict::Drop
    } else {
        Verdict::Accept
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drop_telnet() {
        assert_eq!(inspect_port(23), Verdict::Drop);
    }
}
