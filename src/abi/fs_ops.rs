use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct FsOps {
    files: BTreeMap<String, Vec<u8>>,
}

impl FsOps {
    pub fn create(&mut self, path: &str) -> i64 {
        if self.files.contains_key(path) {
            return -17; // EEXIST
        }
        self.files.insert(path.to_string(), Vec::new());
        0
    }

    pub fn unlink(&mut self, path: &str) -> i64 {
        if self.files.remove(path).is_some() {
            0
        } else {
            -2 // ENOENT
        }
    }

    pub fn rename(&mut self, old: &str, new: &str) -> i64 {
        if let Some(v) = self.files.remove(old) {
            self.files.insert(new.to_string(), v);
            0
        } else {
            -2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_rename_unlink() {
        let mut f = FsOps::default();
        assert_eq!(f.create("/a"), 0);
        assert_eq!(f.rename("/a", "/b"), 0);
        assert_eq!(f.unlink("/b"), 0);
    }
}
