pub mod black;
pub mod white;

use super::{PieceExt, Verify};
use crate::{Board, EIGHTH_RANK, FIRST_RANK};
use api::Square;
use bitboard::Bitboard;
use black::BlackPawn;
use white::WhitePawn;

#[derive(Debug)]
pub enum Pawn {
    Black(BlackPawn),
    White(WhitePawn),
}

pub trait PawnExt: PieceExt {
    fn promotion_mask() -> Bitboard {
        FIRST_RANK | EIGHTH_RANK
    }
}

impl PawnExt for Pawn {}
impl PieceExt for Pawn {
    fn is_illegal_move(&self, source: Square, destination: Square, board: Board) -> bool {
        true
    }
}

impl Verify for Pawn {
    fn verify(&self, source: Square, destination: Square, board: Board) -> Result<u16, ()> {
        match self {
            Pawn::Black(pawn) => pawn.verify(source, destination, board),
            Pawn::White(pawn) => pawn.verify(source, destination, board),
        }
    }
}
