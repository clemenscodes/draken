pub mod black;
pub mod white;

use black::BlackPawn;
use white::WhitePawn;

use crate::March;

use super::PieceExt;

#[derive(Debug)]
pub enum Pawn {
    Black(BlackPawn),
    White(WhitePawn),
}

impl From<WhitePawn> for Pawn {
    fn from(v: WhitePawn) -> Self {
        Self::White(v)
    }
}

impl From<BlackPawn> for Pawn {
    fn from(v: BlackPawn) -> Self {
        Self::Black(v)
    }
}

pub trait PawnExt: PieceExt {}

impl PawnExt for Pawn {}
impl PieceExt for Pawn {}

impl March for Pawn {
    fn march(&self, source: api::Square, destination: api::Square) -> Result<u16, ()> {
        todo!()
    }
}
