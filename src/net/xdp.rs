#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdpAction {
    Pass,
    Drop,
}
pub fn run_xdp(len: usize) -> XdpAction {
    if len == 0 {
        XdpAction::Drop
    } else {
        XdpAction::Pass
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn drop_empty() {
        assert_eq!(run_xdp(0), XdpAction::Drop);
    }
}
