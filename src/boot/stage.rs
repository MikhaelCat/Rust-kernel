#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootStage {
    Reset,
    Setup,
    LoadKernel,
    Jump,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stage_enum() {
        assert_eq!(BootStage::Reset as u8, 0);
    }
}
