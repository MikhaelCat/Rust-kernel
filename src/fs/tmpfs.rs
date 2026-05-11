use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct TmpFs {
    files: u64,
    bytes: usize,
    nodes: BTreeMap<String, Vec<u8>>,
}
impl TmpFs {
    pub fn create(&mut self) {
        self.create_file("/tmp/unnamed", &[]);
    }

    pub fn create_file(&mut self, path: &str, data: &[u8]) {
        if let Some(prev) = self.nodes.insert(path.to_string(), data.to_vec()) {
            self.bytes = self.bytes.saturating_sub(prev.len());
        } else {
            self.files += 1;
        }
        self.bytes += data.len();
    }

    pub fn read_file(&self, path: &str) -> Option<Vec<u8>> {
        self.nodes.get(path).cloned()
    }

    pub fn remove_file(&mut self, path: &str) -> bool {
        if let Some(prev) = self.nodes.remove(path) {
            self.files = self.files.saturating_sub(1);
            self.bytes = self.bytes.saturating_sub(prev.len());
            true
        } else {
            false
        }
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }

    pub fn entries(&self) -> usize {
        self.nodes.len()
    }

    pub fn sync_from_vfs_file(&mut self, path: &str, data: &[u8]) {
        self.create_file(path, data);
    }

    pub fn apply_to_vfs_file(&self, path: &str) -> Option<&[u8]> {
        self.nodes.get(path).map(Vec::as_slice)
    }

    pub fn create_legacy(&mut self) {
        self.files += 1;
    }
    pub fn files(&self) -> u64 {
        self.files
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_file() {
        let mut t = TmpFs::default();
        t.create_legacy();
        assert_eq!(t.files(), 1);
    }

    #[test]
    fn tmpfs_file_lifecycle() {
        let mut t = TmpFs::default();
        t.create_file("/tmp/a", b"abc");
        t.create_file("/tmp/b", b"zz");
        assert_eq!(t.entries(), 2);
        assert_eq!(t.bytes(), 5);
        assert_eq!(t.read_file("/tmp/a").as_deref(), Some(&b"abc"[..]));
        assert!(t.remove_file("/tmp/b"));
        assert_eq!(t.entries(), 1);
    }
}
