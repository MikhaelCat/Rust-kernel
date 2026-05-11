pub fn sanitize_cmdline(raw: &str) -> String {
    raw.split_whitespace()
        .filter(|t| !t.starts_with("secret="))
        .collect::<Vec<_>>()
        .join(" ")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn strip_secret() {
        assert_eq!(sanitize_cmdline("a=1 secret=2 b=3"), "a=1 b=3");
    }
}
