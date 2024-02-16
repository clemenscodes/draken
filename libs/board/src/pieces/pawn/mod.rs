pub mod black;
pub mod white;

use super::{Piece, PieceExt};
use crate::{
    moves::{
        encoded_move::EncodedMove,
        irreversible::{
            capture::CaptureMove,
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
        Move, MoveExt,
    },
    Board, Verify, BOARD_SIZE, EIGHTH_FILE, EIGHTH_RANK, FIRST_FILE, FIRST_RANK,
};
use api::{ForsythEdwardsNotationExt, Square};
use bitboard::{Bitboard, BitboardExt};
use black::BlackPawn;
use std::{error::Error, fmt::Display};
use white::WhitePawn;

#[derive(Debug)]
pub enum Pawn {
    Black(BlackPawn),
    White(WhitePawn),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PawnError {
    Illegal,
    InvalidPromotion,
}

impl Display for PawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PawnError::Illegal => write!(f, "Illegal pawn move"),
            PawnError::InvalidPromotion => write!(f, "Invalid promotion"),
        }
    }
}

impl Error for PawnError {}

pub trait PawnExt: PieceExt {
    fn promotion_mask() -> Bitboard {
        FIRST_RANK | EIGHTH_RANK
    }
    fn west_attack_mask() -> Bitboard {
        !EIGHTH_FILE
    }
    fn east_attack_mask() -> Bitboard {
        !FIRST_FILE
    }
    fn get_west_attacks(&self, pawns: Bitboard) -> Bitboard;
    fn get_east_attacks(&self, pawns: Bitboard) -> Bitboard;
    fn get_attacking_pawns(&self, board: Board) -> Bitboard;
    fn get_single_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard;
    fn get_double_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard;
    fn get_single_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard;
    fn get_double_pushable_pawns(&self, empty_squares: Bitboard) -> Bitboard;
    fn get_promotion_pieces(&self) -> [Piece; 4];
    fn march(&mut self, _source: Square, _destination: Square, _board: &mut Board) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn get_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        self.get_single_push_targets(pawn, empty_squares) | self.get_double_push_targets(pawn, empty_squares)
    }
    fn get_targets(&self, pawn: Bitboard, board: Board) -> Bitboard {
        self.get_push_targets(pawn, board.pieces().empty_squares()) | self.get_attacks(pawn, board)
    }
    fn push(&self, source: Square, destination: Square) -> Result<u16, Box<dyn Error>> {
        let source_index: u8 = source.into();
        let destination_index: u8 = destination.into();
        let difference = destination_index.abs_diff(source_index);
        if difference == BOARD_SIZE {
            return Ok(EncodedMove::from(QuietMove::new(source, destination)).data());
        }
        if difference == 2 * BOARD_SIZE {
            Ok(EncodedMove::from(DoublePushMove::new(source, destination)).data())
        } else {
            Err(Box::new(PawnError::Illegal))
        }
    }
    fn promote(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        if Move::is_capture(destination, board) {
            self.make_promotion_capture(source, destination, promotion)
        } else {
            self.make_promotion(source, destination, promotion)
        }
    }
    fn make_promotion(&self, source: Square, destination: Square, promotion: Option<char>) -> Result<u16, Box<dyn Error>> {
        promotion
            .map(|piece| match piece {
                'q' => Ok(EncodedMove::from(QueenPromotionMove::new(source, destination)).data()),
                'r' => Ok(EncodedMove::from(RookPromotionMove::new(source, destination)).data()),
                'n' => Ok(EncodedMove::from(KnightPromotionMove::new(source, destination)).data()),
                'b' => Ok(EncodedMove::from(BishopPromotionMove::new(source, destination)).data()),
                _ => Err(Box::new(PawnError::InvalidPromotion))?,
            })
            .ok_or_else(|| Box::new(PawnError::InvalidPromotion))?
    }
    fn make_promotion_capture(&self, source: Square, destination: Square, promotion: Option<char>) -> Result<u16, Box<dyn Error>> {
        promotion
            .map(|piece| match piece {
                'q' => Ok(EncodedMove::from(QueenPromotionCaptureMove::new(source, destination)).data()),
                'r' => Ok(EncodedMove::from(RookPromotionCaptureMove::new(source, destination)).data()),
                'n' => Ok(EncodedMove::from(KnightPromotionCaptureMove::new(source, destination)).data()),
                'b' => Ok(EncodedMove::from(BishopPromotionCaptureMove::new(source, destination)).data()),
                _ => Err(Box::new(PawnError::InvalidPromotion))?,
            })
            .ok_or_else(|| Box::new(PawnError::InvalidPromotion))?
    }
}

impl From<WhitePawn> for Pawn {
    fn from(v: WhitePawn) -> Self {
        Self::White(v)
    }
}

impl From<BlackPawn> for Pawn {
    fn from(v: BlackPawn) -> Self {
        Self::Black(v)
    }
}

impl Verify for Pawn {
    fn verify(&self, source: Square, destination: Square, promotion: Option<char>, board: Board) -> Result<u16, Box<dyn Error>> {
        let pawn = Bitboard::get_single_bit(source.into());
        if !Bitboard::check_bit(self.get_targets(pawn, board), destination.into()) {
            return Err(Box::new(PawnError::Illegal));
        }
        if Move::is_promotion(destination) {
            return self.promote(source, destination, promotion, board);
        }
        if Move::is_enpassant(destination, board) {
            return Ok(EncodedMove::from(EnPassantMove::new(source, destination)).data());
        }
        if Move::is_capture(destination, board) {
            return Ok(EncodedMove::from(CaptureMove::new(source, destination)).data());
        }
        self.push(source, destination)
    }
}

impl PieceExt for Pawn {
    fn get_attacks(&self, piece: Bitboard, board: Board) -> Bitboard {
        let enpassant_mask = board.fen().enpassant_mask();
        let west_attacks = self.get_west_attacks(piece);
        let east_attacks = self.get_east_attacks(piece);
        let regular_attacks = west_attacks | east_attacks;
        let enemy_pieces: Bitboard = if board.fen().is_white() {
            board.pieces().black_pieces().into()
        } else {
            board.pieces().white_pieces().into()
        };
        let mut enemy_attacks = regular_attacks & enemy_pieces;
        if regular_attacks.self_overlap(enpassant_mask) {
            enemy_attacks |= enpassant_mask;
        }
        self.remove_friendly_pieces(enemy_attacks, board)
    }
}

impl PawnExt for Pawn {
    fn get_promotion_pieces(&self) -> [Piece; 4] {
        match self {
            Pawn::Black(pawn) => pawn.get_promotion_pieces(),
            Pawn::White(pawn) => pawn.get_promotion_pieces(),
        }
    }

    fn get_west_attacks(&self, pawns: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_west_attacks(pawns),
            Pawn::White(pawn) => pawn.get_west_attacks(pawns),
        }
    }

    fn get_east_attacks(&self, pawns: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_east_attacks(pawns),
            Pawn::White(pawn) => pawn.get_east_attacks(pawns),
        }
    }

    fn get_attacking_pawns(&self, board: Board) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_attacking_pawns(board),
            Pawn::White(pawn) => pawn.get_attacking_pawns(board),
        }
    }

    fn get_single_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(p) => p.get_single_push_targets(pawn, empty_squares),
            Pawn::White(p) => p.get_single_push_targets(pawn, empty_squares),
        }
    }

    fn get_double_push_targets(&self, pawn: Bitboard, empty_squares: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(p) => p.get_double_push_targets(pawn, empty_squares),
            Pawn::White(p) => p.get_double_push_targets(pawn, empty_squares),
        }
    }

    fn get_single_pushable_pawns(&self, empty_squres: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_single_pushable_pawns(empty_squres),
            Pawn::White(pawn) => pawn.get_single_pushable_pawns(empty_squres),
        }
    }

    fn get_double_pushable_pawns(&self, empty_squres: Bitboard) -> Bitboard {
        match self {
            Pawn::Black(pawn) => pawn.get_double_pushable_pawns(empty_squres),
            Pawn::White(pawn) => pawn.get_double_pushable_pawns(empty_squres),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fen::ForsythEdwardsNotation,
        moves::{encoded_move::EncodedMoveExt, QUEEN_PROMOTION_CAPTURE},
    };
    use api::Square::*;

    #[test]
    fn test_promote() {
        let board = Board::from(ForsythEdwardsNotation::try_from("rnbqkbnr/pppp2pP/4p3/8/8/5p2/PPPPPP1P/RNBQKBNR w KQkq - 0 5").unwrap());
        let pawn = Pawn::from(WhitePawn::default());
        let data = pawn.promote(H7, G8, Some('q'), board).unwrap();
        let encoded_move = EncodedMove::new(data);
        assert_eq!(QUEEN_PROMOTION_CAPTURE, encoded_move.kind());
    }
}
