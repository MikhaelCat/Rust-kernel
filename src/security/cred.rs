#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cred {
    pub uid: u32,
    pub gid: u32,
}

impl Cred {
    pub fn root() -> Self {
        Self { uid: 0, gid: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn root_cred() {
        let c = Cred::root();
        assert_eq!(c.uid, 0);
    }
}
