pub mod capture;
pub mod castle;
pub mod pawn;

use super::MoveExt;
use crate::coordinates::Coordinates;
use board::Board;
use capture::CaptureMove;
use castle::CastleMove;
use pawn::PawnMove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrreversibleMove {
    Capture(CaptureMove),
    Pawn(PawnMove),
    Castle(CastleMove),
}

pub trait IrreversibleMoveExt: MoveExt {}

impl IrreversibleMoveExt for IrreversibleMove {}

impl MoveExt for IrreversibleMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            IrreversibleMove::Capture(capture) => capture.coordinates(),
            IrreversibleMove::Pawn(pawn) => pawn.coordinates(),
            IrreversibleMove::Castle(castle) => castle.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) {
        match *self {
            IrreversibleMove::Capture(capture) => capture.march(board),
            IrreversibleMove::Pawn(pawn) => pawn.march(board),
            IrreversibleMove::Castle(castle) => castle.march(board),
        }
    }
}
