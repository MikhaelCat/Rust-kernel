#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zone {
    Dma,
    Normal,
    HighMem,
}
pub fn default_zone() -> Zone {
    Zone::Normal
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_is_normal() {
        assert_eq!(default_zone(), Zone::Normal);
    }
}
