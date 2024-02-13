pub mod black;
pub mod white;

use super::PieceExt;
use crate::{Board, Verify};
use api::Square;
use bitboard::Bitboard;
use black::BlackBishop;
use white::WhiteBishop;

#[derive(Debug)]
pub enum Bishop {
    Black(BlackBishop),
    White(WhiteBishop),
}

pub trait BishopExt: PieceExt {}

impl From<BlackBishop> for Bishop {
    fn from(v: BlackBishop) -> Self {
        Self::Black(v)
    }
}

impl From<WhiteBishop> for Bishop {
    fn from(v: WhiteBishop) -> Self {
        Self::White(v)
    }
}

impl PieceExt for Bishop {
    fn is_illegal_move(&self, _source: Square, _destination: Square, _board: Board) -> bool {
        todo!()
    }

    fn get_attacks(&self, _piece: Bitboard, _board: Board) -> bitboard::Bitboard {
        todo!()
    }
}

impl Verify for Bishop {
    fn verify(&self, _source: Square, _destination: Square, _board: Board) -> Result<u16, ()> {
        todo!()
    }
}

impl BishopExt for Bishop {}
