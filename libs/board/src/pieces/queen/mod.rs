pub mod black;
pub mod white;

use super::PieceExt;
use crate::{Board, Verify};
use api::Square;
use bitboard::Bitboard;
use black::BlackQueen;
use std::error::Error;
use white::WhiteQueen;

#[derive(Debug)]
pub enum Queen {
    Black(BlackQueen),
    White(WhiteQueen),
}

pub trait QueenExt: PieceExt {}

impl From<WhiteQueen> for Queen {
    fn from(v: WhiteQueen) -> Self {
        Self::White(v)
    }
}

impl From<BlackQueen> for Queen {
    fn from(v: BlackQueen) -> Self {
        Self::Black(v)
    }
}

impl PieceExt for Queen {
    fn is_illegal_move(&self, _source: Square, _destination: Square, _board: Board) -> bool {
        todo!()
    }

    fn get_attacks(&self, _piece: Bitboard, _board: Board) -> bitboard::Bitboard {
        todo!()
    }
}

impl Verify for Queen {
    fn verify(&self, _source: Square, _destination: Square, _promotion: Option<char>, _board: Board) -> Result<u16, Box<dyn Error>> {
        todo!()
    }
}

impl QueenExt for Queen {}
