use std::fs;
use std::path::Path;

pub fn write_report(path: &Path, body: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_report_file() {
        let p = Path::new("target/test-report.txt");
        write_report(p, "ok=true").expect("write failed");
        let c = fs::read_to_string(p).expect("read failed");
        assert_eq!(c, "ok=true");
    }
}
