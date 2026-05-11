#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockDevice {
    pub name: String,
    pub block_size: u32,
}
impl BlockDevice {
    pub fn new(name: &str, block_size: u32) -> Self {
        Self {
            name: name.to_string(),
            block_size,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dev_new() {
        assert_eq!(BlockDevice::new("sda", 512).block_size, 512);
    }
}
