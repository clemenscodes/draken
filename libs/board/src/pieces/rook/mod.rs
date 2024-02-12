pub mod black;
pub mod white;

use super::PieceExt;
use crate::{Board, Verify};
use api::Square;
use bitboard::Bitboard;
use black::BlackRook;
use white::WhiteRook;

#[derive(Debug)]
pub enum Rook {
    Black(BlackRook),
    White(WhiteRook),
}

pub trait RookExt: PieceExt {}

impl From<WhiteRook> for Rook {
    fn from(v: WhiteRook) -> Self {
        Self::White(v)
    }
}

impl From<BlackRook> for Rook {
    fn from(v: BlackRook) -> Self {
        Self::Black(v)
    }
}

impl PieceExt for Rook {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        todo!()
    }

    fn get_attacks(&self, piece: Bitboard, board: Board) -> bitboard::Bitboard {
        todo!()
    }
}

impl Verify for Rook {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!()
    }
}

impl RookExt for Rook {}
