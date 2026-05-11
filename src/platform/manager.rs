use super::board::Board;
use super::error::PlatformError;
use super::firmware::Firmware;

#[derive(Debug)]
pub struct PlatformManager {
    board: Board,
    firmware: Firmware,
}

impl PlatformManager {
    pub fn new(board: Board) -> Self {
        Self {
            board,
            firmware: Firmware::default(),
        }
    }

    pub fn load_firmware(&mut self) {
        self.firmware.load();
    }

    pub fn boot_ready(&self) -> Result<&str, PlatformError> {
        if self.firmware.loaded() {
            Ok(self.board.name)
        } else {
            Err(PlatformError::FirmwareNotLoaded)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requires_firmware() {
        let mut p = PlatformManager::new(Board::generic());
        assert_eq!(p.boot_ready(), Err(PlatformError::FirmwareNotLoaded));
        p.load_firmware();
        assert_eq!(p.boot_ready().expect("ready"), "generic");
    }
}
