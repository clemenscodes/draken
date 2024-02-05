use std::fmt::Display;

use api::Square;

pub const QUIET_MOVE: u16 = 0b0000;
pub const DOUBLE_PAWN_PUSH: u16 = 0b0001;
pub const KING_CASTLE: u16 = 0b0010;
pub const QUEEN_CASTLE: u16 = 0b0011;
pub const CAPTURE: u16 = 0b0100;
pub const EP_CAPTURE: u16 = 0b0101;
pub const KNIGHT_PROMOTION: u16 = 0b1000;
pub const BISHOP_PROMOTION: u16 = 0b1001;
pub const ROOK_PROMOTION: u16 = 0b1010;
pub const QUEEN_PROMOTION: u16 = 0b1011;
pub const KNIGHT_PROMOTION_CAPTURE: u16 = 0b1100;
pub const BISHOP_PROMOTION_CAPTURE: u16 = 0b1101;
pub const ROOK_PROMOTION_CAPTURE: u16 = 0b1110;
pub const QUEEN_PROMOTION_CAPTURE: u16 = 0b1111;
pub const SOURCE_MASK: u16 = 0b1111110000000000;
pub const DESTINATION_MASK: u16 = 0b0000001111110000;
pub const KIND_MASK: u16 = 0b0000000000001111;

/// bits 0-5 bits store the source
/// bits 6-11 bits store the destination
/// bits 12-15 store the kind
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EncodedMove {
    data: u16,
}

impl EncodedMove {
    pub fn new(data: u16) -> Self {
        Self { data }
    }

    pub fn data(&self) -> u16 {
        self.data
    }
}

impl Default for EncodedMove {
    fn default() -> Self {
        Self { data: u16::MIN }
    }
}

impl Display for EncodedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data())
    }
}

pub trait EncodedMoveExt {
    fn source(&self) -> Square;
    fn destination(&self) -> Square;
    fn kind(&self) -> u16;
}

impl EncodedMoveExt for EncodedMove {
    fn source(&self) -> Square {
        let bits_remaining_to_start = 10;
        Square::from((self.data & SOURCE_MASK) >> bits_remaining_to_start)
    }

    fn destination(&self) -> Square {
        let bits_remaining_to_start = 4;
        Square::from((self.data & DESTINATION_MASK) >> bits_remaining_to_start)
    }

    fn kind(&self) -> u16 {
        self.data & KIND_MASK
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use api::Square::*;

    #[test]
    fn test_source() {
        let move_data = EncodedMove::new(0b0000010000000000);
        assert_eq!(move_data.source(), B1);
    }

    #[test]
    fn test_destination() {
        let move_data = EncodedMove::new(0b0000000001000000);
        assert_eq!(move_data.destination(), E1);
    }

    #[test]
    fn test_kind() {
        let move_data = EncodedMove::new(QUIET_MOVE);
        assert_eq!(move_data.kind(), QUIET_MOVE);
        let move_data = EncodedMove::new(KNIGHT_PROMOTION_CAPTURE);
        assert_eq!(move_data.kind(), KNIGHT_PROMOTION_CAPTURE);
    }
}
