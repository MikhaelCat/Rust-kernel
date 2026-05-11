#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootLink {
    pub idt_ready: bool,
    pub paging_ready: bool,
}

impl BootLink {
    pub fn ready(&self) -> bool {
        self.idt_ready && self.paging_ready
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_ready() {
        assert!(
            BootLink {
                idt_ready: true,
                paging_ready: true
            }
            .ready()
        );
        assert!(
            !BootLink {
                idt_ready: true,
                paging_ready: false
            }
            .ready()
        );
    }
}
