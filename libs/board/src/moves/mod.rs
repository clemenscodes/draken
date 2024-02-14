pub mod coordinates;
pub mod encoded_move;
pub mod irreversible;
pub mod list;
pub mod reversible;

use std::{error::Error, fmt::Display};

use crate::{
    fen::active_color::ActiveColorExt,
    pieces::{piece::Piece, Pawn, PawnExt},
    Board,
};
use api::ForsythEdwardsNotationExt;
use bitboard::{Bitboard, BitboardExt};
use coordinates::Coordinates;
use irreversible::IrreversibleMove;
use reversible::ReversibleMove;

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

const SOURCE_SHIFT: usize = 10;
const DESTINATION_SHIFT: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Reversible(ReversibleMove),
    Irreversible(IrreversibleMove),
}

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    OpponentsPiece,
}

impl Display for MoveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveError::OpponentsPiece => write!(f, "Can not move opponents piece"),
        }
    }
}

impl Error for MoveError {}

pub trait MoveExt {
    fn coordinates(&self) -> Coordinates;
    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>>;
    fn piece(&self, board: &mut Board) -> Piece {
        board.get_piece_mut(self.coordinates().source()).expect("No piece on {source}")
    }
    fn is_capture(&self, board: Board) -> bool {
        let destination: Bitboard = self.coordinates().destination().into();
        let pieces: Bitboard = if board.fen().is_white() {
            board.pieces().black_pieces().into()
        } else {
            board.pieces().white_pieces().into()
        };
        Bitboard::overlap(destination, pieces)
    }
    fn is_promotion(&self) -> bool {
        let destination: Bitboard = self.coordinates().destination().into();
        let mask = Pawn::promotion_mask();
        Bitboard::overlap(destination, mask)
    }
    fn is_enpassant(&self, board: Board) -> bool {
        let mask = board.fen().enpassant_mask();
        let destination: Bitboard = self.coordinates().destination().into();
        Bitboard::overlap(destination, mask)
    }
    fn verify(&self, board: &mut Board) -> bool {
        let player: Bitboard = if board.fen().is_white() {
            board.pieces().white_pieces().into()
        } else {
            board.pieces().black_pieces().into()
        };
        let source = self.coordinates().source();
        let piece = Bitboard::get_single_bit(source.into());
        Bitboard::overlap(player, piece)
    }
    fn switch(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        if !self.verify(board) {
            return Err(Box::new(MoveError::OpponentsPiece));
        }
        board.fen_mut().active_color_mut().switch();
        Ok(())
    }
}

pub trait Encode: MoveExt {
    fn encode(&self, kind_mask: u16) -> u16 {
        let source_index: u16 = self.coordinates().source().into();
        let destination_index: u16 = self.coordinates().destination().into();
        let source = source_index << SOURCE_SHIFT;
        let destination = destination_index << DESTINATION_SHIFT;
        source | destination | kind_mask
    }
}

impl MoveExt for Move {
    fn coordinates(&self) -> Coordinates {
        match *self {
            Move::Reversible(reversible) => reversible.coordinates(),
            Move::Irreversible(irreversible) => irreversible.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        match *self {
            Move::Reversible(reversible) => reversible.march(board),
            Move::Irreversible(irreversible) => irreversible.march(board),
        }
    }
}
