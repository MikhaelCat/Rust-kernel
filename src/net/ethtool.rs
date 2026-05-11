#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthtoolInfo {
    pub driver: String,
    pub speed_mbps: u32,
}

impl EthtoolInfo {
    pub fn new(driver: &str, speed_mbps: u32) -> Self {
        Self {
            driver: driver.to_string(),
            speed_mbps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_fields() {
        let e = EthtoolInfo::new("e1000", 1000);
        assert_eq!(e.speed_mbps, 1000);
    }
}
