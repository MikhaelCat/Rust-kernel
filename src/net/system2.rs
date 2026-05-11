use super::NetSystem;

pub fn smoke() -> bool {
    let mut n = NetSystem::new();
    n.bootstrap();
    n.smoke_send() > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_ok() {
        assert!(smoke());
    }
}
