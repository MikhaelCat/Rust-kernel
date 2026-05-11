use super::KernelSystem;

pub fn smoke() -> bool {
    let mut k = KernelSystem::new();
    k.bootstrap();
    k.tick() == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_ok() {
        assert!(smoke());
    }
}
