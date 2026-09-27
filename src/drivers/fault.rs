//! Driver fault handling

#[derive(Debug)]
pub struct DriverFault {
    pub message: String,
}

impl DriverFault {
    pub fn new(msg: &str) -> Self {
        Self { message: msg.to_string() }
    }
}
