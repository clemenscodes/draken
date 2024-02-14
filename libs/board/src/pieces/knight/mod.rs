pub mod black;
pub mod white;

use std::error::Error;

use super::PieceExt;
use crate::{Board, Verify};
use api::Square;
use bitboard::Bitboard;
use black::BlackKnight;
use white::WhiteKnight;

#[derive(Debug)]
pub enum Knight {
    Black(BlackKnight),
    White(WhiteKnight),
}

pub trait KnightExt: PieceExt {}

impl From<WhiteKnight> for Knight {
    fn from(v: WhiteKnight) -> Self {
        Self::White(v)
    }
}

impl From<BlackKnight> for Knight {
    fn from(v: BlackKnight) -> Self {
        Self::Black(v)
    }
}

impl PieceExt for Knight {
    fn is_illegal_move(&self, _source: Square, _destination: Square, _board: Board) -> bool {
        todo!()
    }

    fn get_attacks(&self, _piece: Bitboard, _board: Board) -> bitboard::Bitboard {
        todo!()
    }
}

impl Verify for Knight {
    fn verify(&self, _source: Square, _destination: Square, _promotion: Option<char>, _board: Board) -> Result<u16, Box<dyn Error>> {
        todo!()
    }
}

impl KnightExt for Knight {}
