pub mod black;
pub mod white;

use black::BlackBishop;
use white::WhiteBishop;

use crate::March;

use super::PieceExt;

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
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        todo!()
    }
}
