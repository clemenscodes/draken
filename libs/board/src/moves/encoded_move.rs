use super::{
    irreversible::{
        capture::CaptureMove,
        castle::{king::KingCastleMove, queen::QueenCastleMove},
        pawn::{
            enpassant::EnPassantMove,
            promotion::{
                bishop::BishopPromotionMove,
                capture::{
                    bishop::BishopPromotionCaptureMove, knight::KnightPromotionCaptureMove, queen::QueenPromotionCaptureMove,
                    rook::RookPromotionCaptureMove,
                },
                knight::KnightPromotionMove,
                queen::QueenPromotionMove,
                rook::RookPromotionMove,
            },
            push::DoublePushMove,
        },
    },
    reversible::quiet::QuietMove,
    *,
};
use api::Square;
use std::fmt::{Debug, Display, Error};

const SOURCE_MASK: u16 = 0b1111110000000000;
const DESTINATION_MASK: u16 = 0b0000001111110000;
const KIND_MASK: u16 = 0b0000000000001111;

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct EncodedMove {
    data: u16,
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
        Self::new(u16::MIN)
    }
}

impl From<QuietMove> for EncodedMove {
    fn from(value: QuietMove) -> Self {
        Self::new(value.encode(QUIET_MOVE))
    }
}

impl From<CaptureMove> for EncodedMove {
    fn from(value: CaptureMove) -> Self {
        Self::new(value.encode(CAPTURE))
    }
}

impl From<DoublePushMove> for EncodedMove {
    fn from(value: DoublePushMove) -> Self {
        Self::new(value.encode(DOUBLE_PAWN_PUSH))
    }
}

impl From<KingCastleMove> for EncodedMove {
    fn from(value: KingCastleMove) -> Self {
        Self::new(value.encode(KING_CASTLE))
    }
}

impl From<QueenCastleMove> for EncodedMove {
    fn from(value: QueenCastleMove) -> Self {
        Self::new(value.encode(QUEEN_CASTLE))
    }
}

impl From<EnPassantMove> for EncodedMove {
    fn from(value: EnPassantMove) -> Self {
        Self::new(value.encode(ENPASSANT))
    }
}

impl From<KnightPromotionMove> for EncodedMove {
    fn from(value: KnightPromotionMove) -> Self {
        Self::new(value.encode(KNIGHT_PROMOTION))
    }
}

impl From<BishopPromotionMove> for EncodedMove {
    fn from(value: BishopPromotionMove) -> Self {
        Self::new(value.encode(BISHOP_PROMOTION))
    }
}

impl From<RookPromotionMove> for EncodedMove {
    fn from(value: RookPromotionMove) -> Self {
        Self::new(value.encode(ROOK_PROMOTION))
    }
}

impl From<QueenPromotionMove> for EncodedMove {
    fn from(value: QueenPromotionMove) -> Self {
        Self::new(value.encode(QUEEN_PROMOTION))
    }
}

impl From<KnightPromotionCaptureMove> for EncodedMove {
    fn from(value: KnightPromotionCaptureMove) -> Self {
        Self::new(value.encode(KNIGHT_PROMOTION_CAPTURE))
    }
}

impl From<BishopPromotionCaptureMove> for EncodedMove {
    fn from(value: BishopPromotionCaptureMove) -> Self {
        Self::new(value.encode(BISHOP_PROMOTION_CAPTURE))
    }
}

impl From<RookPromotionCaptureMove> for EncodedMove {
    fn from(value: RookPromotionCaptureMove) -> Self {
        Self::new(value.encode(ROOK_PROMOTION_CAPTURE))
    }
}

impl From<QueenPromotionCaptureMove> for EncodedMove {
    fn from(value: QueenPromotionCaptureMove) -> Self {
        Self::new(value.encode(QUEEN_PROMOTION_CAPTURE))
    }
}

impl Display for EncodedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.source();
        let destination = self.destination();
        let encoded_move: String = match self.kind() {
            QUIET_MOVE => QuietMove::new(source, destination).to_string(),
            DOUBLE_PAWN_PUSH => DoublePushMove::new(source, destination).to_string(),
            KING_CASTLE => KingCastleMove::new(source, destination).to_string(),
            QUEEN_CASTLE => QueenCastleMove::new(source, destination).to_string(),
            CAPTURE => CaptureMove::new(source, destination).to_string(),
            ENPASSANT => EnPassantMove::new(source, destination).to_string(),
            KNIGHT_PROMOTION => KnightPromotionMove::new(source, destination).to_string(),
            BISHOP_PROMOTION => BishopPromotionMove::new(source, destination).to_string(),
            ROOK_PROMOTION => RookPromotionMove::new(source, destination).to_string(),
            QUEEN_PROMOTION => QueenPromotionMove::new(source, destination).to_string(),
            KNIGHT_PROMOTION_CAPTURE => KnightPromotionCaptureMove::new(source, destination).to_string(),
            BISHOP_PROMOTION_CAPTURE => BishopPromotionCaptureMove::new(source, destination).to_string(),
            ROOK_PROMOTION_CAPTURE => RookPromotionCaptureMove::new(source, destination).to_string(),
            QUEEN_PROMOTION_CAPTURE => QueenPromotionCaptureMove::new(source, destination).to_string(),
            _ => return Err(Error),
        };
        write!(f, "{encoded_move}")
    }
}

impl Debug for EncodedMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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

    #[test]
    fn test_double_push_move_into_encoded_move() {
        let double_push_move = DoublePushMove::new(E2, E4);
        let encoded_move: EncodedMove = double_push_move.into();
        let expected_source_index: u16 = double_push_move.coordinates().source().into();
        let expected_destination_index: u16 = double_push_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | DOUBLE_PAWN_PUSH;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_king_castle_move_into_encoded_move() {
        let king_castle_move = KingCastleMove::new(E2, E4);
        let encoded_move: EncodedMove = king_castle_move.into();
        let expected_source_index: u16 = king_castle_move.coordinates().source().into();
        let expected_destination_index: u16 = king_castle_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | KING_CASTLE;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_queen_castle_move_into_encoded_move() {
        let queen_castle_move = QueenCastleMove::new(E2, E4);
        let encoded_move: EncodedMove = queen_castle_move.into();
        let expected_source_index: u16 = queen_castle_move.coordinates().source().into();
        let expected_destination_index: u16 = queen_castle_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | QUEEN_CASTLE;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_enpassant_move_into_encoded_move() {
        let enpassant_move = EnPassantMove::new(E2, E4);
        let encoded_move: EncodedMove = enpassant_move.into();
        let expected_source_index: u16 = enpassant_move.coordinates().source().into();
        let expected_destination_index: u16 = enpassant_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | ENPASSANT;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_knight_promotion_move_into_encoded_move() {
        let knight_promotion_move = KnightPromotionMove::new(E2, E4);
        let encoded_move: EncodedMove = knight_promotion_move.into();
        let expected_source_index: u16 = knight_promotion_move.coordinates().source().into();
        let expected_destination_index: u16 = knight_promotion_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | KNIGHT_PROMOTION;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_bishop_promotion_move_into_encoded_move() {
        let bishop_promotion_move = BishopPromotionMove::new(E2, E4);
        let encoded_move: EncodedMove = bishop_promotion_move.into();
        let expected_source_index: u16 = bishop_promotion_move.coordinates().source().into();
        let expected_destination_index: u16 = bishop_promotion_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | BISHOP_PROMOTION;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_rook_promotion_move_into_encoded_move() {
        let rook_promotion_move = RookPromotionMove::new(E2, E4);
        let encoded_move: EncodedMove = rook_promotion_move.into();
        let expected_source_index: u16 = rook_promotion_move.coordinates().source().into();
        let expected_destination_index: u16 = rook_promotion_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | ROOK_PROMOTION;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_queen_promotion_move_into_encoded_move() {
        let queen_promotion_move = QueenPromotionMove::new(E2, E4);
        let encoded_move: EncodedMove = queen_promotion_move.into();
        let expected_source_index: u16 = queen_promotion_move.coordinates().source().into();
        let expected_destination_index: u16 = queen_promotion_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | QUEEN_PROMOTION;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_knight_promotion_capture_move_into_encoded_move() {
        let knight_promotion_capture_move = KnightPromotionCaptureMove::new(E2, E4);
        let encoded_move: EncodedMove = knight_promotion_capture_move.into();
        let expected_source_index: u16 = knight_promotion_capture_move.coordinates().source().into();
        let expected_destination_index: u16 = knight_promotion_capture_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | KNIGHT_PROMOTION_CAPTURE;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_bishop_promotion_capture_move_into_encoded_move() {
        let bishop_promotion_capture_move = BishopPromotionCaptureMove::new(E2, E4);
        let encoded_move: EncodedMove = bishop_promotion_capture_move.into();
        let expected_source_index: u16 = bishop_promotion_capture_move.coordinates().source().into();
        let expected_destination_index: u16 = bishop_promotion_capture_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | BISHOP_PROMOTION_CAPTURE;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_rook_promotion_capture_move_into_encoded_move() {
        let rook_promotion_capture_move = RookPromotionCaptureMove::new(E2, E4);
        let encoded_move: EncodedMove = rook_promotion_capture_move.into();
        let expected_source_index: u16 = rook_promotion_capture_move.coordinates().source().into();
        let expected_destination_index: u16 = rook_promotion_capture_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | ROOK_PROMOTION_CAPTURE;
        assert_eq!(encoded_move.data, expected_data);
    }

    #[test]
    fn test_queen_promotion_capture_move_into_encoded_move() {
        let queen_promotion_capture_move = QueenPromotionCaptureMove::new(E2, E4);
        let encoded_move: EncodedMove = queen_promotion_capture_move.into();
        let expected_source_index: u16 = queen_promotion_capture_move.coordinates().source().into();
        let expected_destination_index: u16 = queen_promotion_capture_move.coordinates().destination().into();
        let expected_source = expected_source_index << SOURCE_SHIFT;
        let expected_destination = expected_destination_index << DESTINATION_SHIFT;
        let expected_data = expected_source | expected_destination | QUEEN_PROMOTION_CAPTURE;
        assert_eq!(encoded_move.data, expected_data);
    }
}
