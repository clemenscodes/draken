pub mod black;
pub mod white;

use black::BlackKing;
use white::WhiteKing;

use crate::March;

use super::PieceExt;

#[derive(Debug)]
pub enum King {
    Black(BlackKing),
    White(WhiteKing),
}

impl From<WhiteKing> for King {
    fn from(v: WhiteKing) -> Self {
        Self::White(v)
    }
}

impl From<BlackKing> for King {
    fn from(v: BlackKing) -> Self {
        Self::Black(v)
    }
}

pub trait KingExt: PieceExt {}

impl KingExt for King {}
impl PieceExt for King {}

impl March for King {
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        todo!()
    }
}
