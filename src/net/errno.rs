pub fn to_errno(disposition: &str) -> i64 {
    match disposition {
        "notconn" => -107,
        "again" => -11,
        "inval" => -22,
        _ => -5,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn errno_map() {
        assert_eq!(to_errno("notconn"), -107);
    }
}
