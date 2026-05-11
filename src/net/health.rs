#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetHealth {
    pub link_up: bool,
}

pub fn check(link_up: bool) -> NetHealth {
    NetHealth { link_up }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_state_reflected() {
        assert!(check(true).link_up);
        assert!(!check(false).link_up);
    }
}
