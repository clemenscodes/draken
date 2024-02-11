pub mod black;
pub mod white;

use super::PieceExt;
use crate::{Board, Verify};
use api::Square;
use black::BlackKnight;
use white::WhiteKnight;

#[derive(Debug)]
pub enum Knight {
    Black(BlackKnight),
    White(WhiteKnight),
}

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

pub trait KnightExt: PieceExt {}

impl PieceExt for Knight {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        todo!()
    }
}

impl Verify for Knight {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!()
    }
}

impl KnightExt for Knight {}
