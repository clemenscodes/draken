pub mod black;
pub mod white;

use crate::Board;

use super::{March, PieceExt};
use api::Square;
use black::BlackBishop;
use white::WhiteBishop;

#[derive(Debug)]
pub enum Bishop {
    Black(BlackBishop),
    White(WhiteBishop),
}

impl From<BlackBishop> for Bishop {
    fn from(v: BlackBishop) -> Self {
        Self::Black(v)
    }
}

impl From<WhiteBishop> for Bishop {
    fn from(v: WhiteBishop) -> Self {
        Self::White(v)
    }
}

pub trait BishopExt: PieceExt {}

impl BishopExt for Bishop {}
impl PieceExt for Bishop {}

impl March for Bishop {
    fn march(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        todo!()
    }
}
