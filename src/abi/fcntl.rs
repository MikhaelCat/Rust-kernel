use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct FcntlTable {
    cloexec: BTreeMap<i64, bool>,
}

impl FcntlTable {
    pub fn set_cloexec(&mut self, fd: i64, val: bool) -> i64 {
        if fd < 0 {
            return -9;
        }
        self.cloexec.insert(fd, val);
        0
    }

    pub fn get_cloexec(&self, fd: i64) -> i64 {
        self.cloexec
            .get(&fd)
            .copied()
            .map(|v| if v { 1 } else { 0 })
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_flag() {
        let mut t = FcntlTable::default();
        assert_eq!(t.set_cloexec(3, true), 0);
        assert_eq!(t.get_cloexec(3), 1);
    }
}
