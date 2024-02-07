pub mod black;
pub mod white;

use super::{PieceExt, Verify};
use crate::Board;
use api::Square;
use black::BlackPawn;
use white::WhitePawn;

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

impl Verify for Pawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        match self {
            Pawn::Black(pawn) => pawn.verify(source, destination, board),
            Pawn::White(pawn) => pawn.verify(source, destination, board),
        }
    }
}
