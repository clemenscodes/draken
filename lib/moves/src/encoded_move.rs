use std::fmt::Display;

use api::Square;

use crate::{
    irreversible::{
        capture::*,
        castle::{king::*, queen::*},
        pawn::{
            enpassant::*,
            promotion::{
                bishop::*,
                capture::{bishop::*, knight::*, queen::*, rook::*},
                knight::*,
                queen::*,
                rook::*,
            },
            push::*,
        },
    },
    reversible::quiet::*,
    Encode,
};

pub const QUIET_MOVE: u16 = 0b0000;
pub const DOUBLE_PAWN_PUSH: u16 = 0b0001;
pub const KING_CASTLE: u16 = 0b0010;
pub const QUEEN_CASTLE: u16 = 0b0011;
pub const CAPTURE: u16 = 0b0100;
pub const ENPASSANT: u16 = 0b0101;
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
pub const SOURCE_SHIFT: usize = 10;
pub const DESTINATION_SHIFT: usize = 4;

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

impl From<QuietMove> for EncodedMove {
    fn from(value: QuietMove) -> Self {
        Self {
            data: value.encode(QUIET_MOVE),
        }
    }
}

impl From<CaptureMove> for EncodedMove {
    fn from(value: CaptureMove) -> Self {
        Self {
            data: value.encode(CAPTURE),
        }
    }
}

impl From<DoublePushMove> for EncodedMove {
    fn from(value: DoublePushMove) -> Self {
        Self {
            data: value.encode(DOUBLE_PAWN_PUSH),
        }
    }
}

impl From<KingCastleMove> for EncodedMove {
    fn from(value: KingCastleMove) -> Self {
        Self {
            data: value.encode(KING_CASTLE),
        }
    }
}

impl From<QueenCastleMove> for EncodedMove {
    fn from(value: QueenCastleMove) -> Self {
        Self {
            data: value.encode(QUEEN_CASTLE),
        }
    }
}

impl From<EnPassantMove> for EncodedMove {
    fn from(value: EnPassantMove) -> Self {
        Self {
            data: value.encode(ENPASSANT),
        }
    }
}

impl From<KnightPromotionMove> for EncodedMove {
    fn from(value: KnightPromotionMove) -> Self {
        Self {
            data: value.encode(KNIGHT_PROMOTION),
        }
    }
}

impl From<BishopPromotionMove> for EncodedMove {
    fn from(value: BishopPromotionMove) -> Self {
        Self {
            data: value.encode(BISHOP_PROMOTION),
        }
    }
}

impl From<RookPromotionMove> for EncodedMove {
    fn from(value: RookPromotionMove) -> Self {
        Self {
            data: value.encode(ROOK_PROMOTION),
        }
    }
}

impl From<QueenPromotionMove> for EncodedMove {
    fn from(value: QueenPromotionMove) -> Self {
        Self {
            data: value.encode(QUEEN_PROMOTION),
        }
    }
}

impl From<KnightPromotionCaptureMove> for EncodedMove {
    fn from(value: KnightPromotionCaptureMove) -> Self {
        Self {
            data: value.encode(KNIGHT_PROMOTION_CAPTURE),
        }
    }
}

impl From<BishopPromotionCaptureMove> for EncodedMove {
    fn from(value: BishopPromotionCaptureMove) -> Self {
        Self {
            data: value.encode(BISHOP_PROMOTION_CAPTURE),
        }
    }
}

impl From<RookPromotionCaptureMove> for EncodedMove {
    fn from(value: RookPromotionCaptureMove) -> Self {
        Self {
            data: value.encode(ROOK_PROMOTION_CAPTURE),
        }
    }
}

impl From<QueenPromotionCaptureMove> for EncodedMove {
    fn from(value: QueenPromotionCaptureMove) -> Self {
        Self {
            data: value.encode(QUEEN_PROMOTION_CAPTURE),
        }
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
        Square::from((self.data & SOURCE_MASK) >> SOURCE_SHIFT)
    }

    fn destination(&self) -> Square {
        Square::from((self.data & DESTINATION_MASK) >> DESTINATION_SHIFT)
    }

    fn kind(&self) -> u16 {
        self.data & KIND_MASK
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
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

    #[test]
    fn test_encoded_move_from_quiet_move() {
        let quiet_move = QuietMove::new(E2, E4);
        let encoded_move: EncodedMove = quiet_move.into();
        let expected_source_index: u16 = quiet_move.coordinates().source().into();
        let expected_destination_index: u16 = quiet_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | QUIET_MOVE;
        assert_eq!(encoded_move.data, expected_data);
    }
}
