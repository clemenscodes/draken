pub mod black;
pub mod white;

use super::PieceExt;
use crate::{Board, Verify};
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
impl PieceExt for Rook {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        todo!()
    }
}

impl Verify for Rook {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!()
    }
}
