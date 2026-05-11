use super::board::Board;
use super::firmware::Firmware;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatResult {
    Compatible,
    Incompatible,
}

pub fn check_board_firmware(board: &Board, fw: &Firmware) -> CompatResult {
    if board.name == "generic" && fw.loaded() {
        CompatResult::Compatible
    } else {
        CompatResult::Incompatible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::board::Board;
    use crate::platform::firmware::Firmware;

    #[test]
    fn generic_board_with_fw_is_compatible() {
        let mut fw = Firmware::default();
        fw.load();
        assert_eq!(
            check_board_firmware(&Board::generic(), &fw),
            CompatResult::Compatible
        );
    }
}
