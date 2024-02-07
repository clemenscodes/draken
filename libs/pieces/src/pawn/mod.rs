pub mod black;
pub mod white;

use black::BlackPawn;
use white::WhitePawn;

use super::PieceExt;

#[derive(Debug)]
pub enum Pawn {
    Black(BlackPawn),
    White(WhitePawn),
}

pub trait PawnExt: PieceExt {}

impl PawnExt for Pawn {}
impl PieceExt for Pawn {}
