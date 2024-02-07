pub mod black;
pub mod white;

use super::{March, PieceExt};
use crate::{fen::ForsythEdwardsNotation, Board};
use api::Square;
use black::BlackRook;
use white::WhiteRook;

#[derive(Debug)]
pub enum Rook {
    Black(BlackRook),
    White(WhiteRook),
}

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

pub trait RookExt: PieceExt {}

impl RookExt for Rook {}
impl PieceExt for Rook {}

impl March for Rook {
    fn march(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!()
    }
}
