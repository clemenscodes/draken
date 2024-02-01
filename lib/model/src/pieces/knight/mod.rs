pub mod black;
pub mod white;

use black::BlackKnight;
use white::WhiteKnight;

use super::PieceExt;

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

impl KnightExt for Knight {}
impl PieceExt for Knight {}
