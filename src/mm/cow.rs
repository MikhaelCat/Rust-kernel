#[derive(Debug, Default)]
pub struct CowTracker {
    copies: u64,
}

impl CowTracker {
    pub fn on_write_shared_page(&mut self) {
        self.copies += 1;
    }

    pub fn copies(&self) -> u64 {
        self.copies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cow_counter() {
        let mut c = CowTracker::default();
        c.on_write_shared_page();
        assert_eq!(c.copies(), 1);
    }
}
