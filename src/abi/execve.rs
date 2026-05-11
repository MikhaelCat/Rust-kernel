#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecImage {
    pub path: String,
    pub argv: Vec<String>,
    pub envp: Vec<String>,
}

impl ExecImage {
    pub fn new(path: &str, argv: &[&str], envp: &[&str]) -> Self {
        Self {
            path: path.to_string(),
            argv: argv.iter().map(|s| s.to_string()).collect(),
            envp: envp.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn sane(&self) -> bool {
        !self.path.is_empty() && !self.argv.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_sane() {
        let i = ExecImage::new("/bin/sh", &["sh"], &["PATH=/bin"]);
        assert!(i.sane());
    }
}
