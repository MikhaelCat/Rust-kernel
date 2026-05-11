#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XtensaBoot {
    pub vectors_ready: bool,
}
impl XtensaBoot {
    pub fn bootstrap() -> Self {
        Self {
            vectors_ready: true,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn xtensa_boot() {
        assert!(XtensaBoot::bootstrap().vectors_ready);
    }
}
