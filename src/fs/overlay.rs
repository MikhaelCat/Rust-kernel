#[derive(Debug, Default)]
pub struct OverlayFs {
    layers: u32,
}
impl OverlayFs {
    pub fn add_layer(&mut self) {
        self.layers += 1;
    }
    pub fn layers(&self) -> u32 {
        self.layers
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_layer() {
        let mut o = OverlayFs::default();
        o.add_layer();
        assert_eq!(o.layers(), 1);
    }
}
