#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mode {
    pub owner_read: bool,
    pub owner_write: bool,
    pub other_read: bool,
    pub other_write: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cred {
    pub uid: u32,
}

pub fn can_read(cred: Cred, owner_uid: u32, mode: Mode) -> bool {
    if cred.uid == 0 {
        return true;
    }
    if cred.uid == owner_uid {
        mode.owner_read
    } else {
        mode.other_read
    }
}

pub fn can_write(cred: Cred, owner_uid: u32, mode: Mode) -> bool {
    if cred.uid == 0 {
        return true;
    }
    if cred.uid == owner_uid {
        mode.owner_write
    } else {
        mode.other_write
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_checks() {
        let m = Mode {
            owner_read: true,
            owner_write: false,
            other_read: false,
            other_write: false,
        };
        let owner = Cred { uid: 1000 };
        let other = Cred { uid: 2000 };
        assert!(can_read(owner, 1000, m));
        assert!(!can_write(owner, 1000, m));
        assert!(!can_read(other, 1000, m));
    }
}
