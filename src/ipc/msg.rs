#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Msg {
    pub typ: u64,
    pub bytes: Vec<u8>,
}
impl Msg {
    pub fn new(typ: u64, bytes: &[u8]) -> Self {
        Self {
            typ,
            bytes: bytes.to_vec(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn msg_new() {
        assert_eq!(Msg::new(1, b"ok").typ, 1);
    }
}
