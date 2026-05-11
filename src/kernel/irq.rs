#[derive(Debug, Default)]
pub struct IrqController {
    handled: u64,
}
impl IrqController {
    pub fn handle_irq(&mut self) {
        self.handled += 1;
    }
    pub fn handled(&self) -> u64 {
        self.handled
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handles_irq() {
        let mut c = IrqController::default();
        c.handle_irq();
        assert_eq!(c.handled(), 1);
    }
}
