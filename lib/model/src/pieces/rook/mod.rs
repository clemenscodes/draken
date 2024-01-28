pub mod black;
pub mod white;

use black::BlackRook;
use white::WhiteRook;

use super::PieceExt;

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
