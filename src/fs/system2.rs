use super::FsSystem;

pub fn smoke() -> bool {
    let mut fs = FsSystem::new();
    fs.bootstrap();
    fs.vfs().is_mounted("/tmp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_ok() {
        assert!(smoke());
    }
}
