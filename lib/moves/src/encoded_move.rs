use std::fmt::Display;

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
