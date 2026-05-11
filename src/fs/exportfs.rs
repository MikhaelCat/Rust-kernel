#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Export {
    pub path: String,
}
impl Export {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn export_new() {
        assert_eq!(Export::new("/srv").path, "/srv");
    }
}
