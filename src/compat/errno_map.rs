pub fn linux_errno(code: i64) -> Option<&'static str> {
    match code {
        -1 => Some("EPERM"),
        -2 => Some("ENOENT"),
        -9 => Some("EBADF"),
        -11 => Some("EAGAIN"),
        -13 => Some("EACCES"),
        -17 => Some("EEXIST"),
        -20 => Some("ENOTDIR"),
        -21 => Some("EISDIR"),
        -22 => Some("EINVAL"),
        -107 => Some("ENOTCONN"),
        _ => None,
    }
}

pub fn errno_name(code: i64) -> &'static str {
    linux_errno(code).unwrap_or("UNKNOWN")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_errno() {
        assert_eq!(errno_name(-9), "EBADF");
        assert_eq!(errno_name(-9999), "UNKNOWN");
    }
}
