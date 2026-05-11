#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtFlags {
    pub r: bool,
    pub w: bool,
    pub x: bool,
}

impl ProtFlags {
    pub fn rw() -> Self {
        Self {
            r: true,
            w: true,
            x: false,
        }
    }

    pub fn rx() -> Self {
        Self {
            r: true,
            w: false,
            x: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
    Exec,
}

pub fn check_access(p: ProtFlags, a: AccessType) -> bool {
    match a {
        AccessType::Read => p.r,
        AccessType::Write => p.w,
        AccessType::Exec => p.x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protection_checks() {
        let rw = ProtFlags::rw();
        assert!(check_access(rw, AccessType::Read));
        assert!(check_access(rw, AccessType::Write));
        assert!(!check_access(rw, AccessType::Exec));
    }
}
