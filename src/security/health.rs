#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityHealth {
    pub enforcing: bool,
}

pub fn check(enforcing: bool) -> SecurityHealth {
    SecurityHealth { enforcing }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_passthrough() {
        assert!(check(true).enforcing);
    }
}
