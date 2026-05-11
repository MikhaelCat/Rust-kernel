#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccelDevice {
    pub name: String,
    pub queues: u32,
}
impl AccelDevice {
    pub fn new(name: &str, queues: u32) -> Self {
        Self {
            name: name.to_string(),
            queues,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accel_new() {
        assert_eq!(AccelDevice::new("amdxdna", 8).queues, 8);
    }
}
