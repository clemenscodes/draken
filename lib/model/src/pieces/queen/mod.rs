pub mod black;
pub mod white;

use black::BlackQueen;
use white::WhiteQueen;

use super::PieceExt;

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
impl PieceExt for Queen {}
