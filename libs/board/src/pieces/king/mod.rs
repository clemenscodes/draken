pub mod black;
pub mod white;

use super::{PieceExt, Verify};
use crate::Board;
use api::Square;
use black::BlackKing;
use white::WhiteKing;

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

impl Verify for King {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!()
    }
}
