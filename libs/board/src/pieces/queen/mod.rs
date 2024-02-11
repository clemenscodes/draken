pub mod black;
pub mod white;

use super::PieceExt;
use crate::{Board, Verify};
use api::Square;
use black::BlackQueen;
use white::WhiteQueen;

#[derive(Debug)]
pub enum Queen {
    Black(BlackQueen),
    White(WhiteQueen),
}

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

pub trait QueenExt: PieceExt {}

impl QueenExt for Queen {}
impl PieceExt for Queen {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        todo!()
    }
}

impl Verify for Queen {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!()
    }
}
