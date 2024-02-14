pub mod capture;
pub mod castle;
pub mod pawn;

use super::{coordinates::Coordinates, MoveExt};
use crate::{fen::half_move_clock::HalfMoveClockExt, Board};
use capture::CaptureMove;
use castle::CastleMove;
use pawn::PawnMove;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrreversibleMove {
    Capture(CaptureMove),
    Pawn(PawnMove),
    Castle(CastleMove),
}

pub trait IrreversibleMoveExt: MoveExt {
    fn make(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        let piece = self.piece(board)?;
        if piece.is_king() {
            board.fen_mut().half_move_clock_mut().increment();
        } else {
            board.fen_mut().half_move_clock_mut().reset()
        }
        self.switch(board)
    }
}

impl MoveExt for IrreversibleMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            IrreversibleMove::Capture(capture) => capture.coordinates(),
            IrreversibleMove::Pawn(pawn) => pawn.coordinates(),
            IrreversibleMove::Castle(castle) => castle.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        match *self {
            IrreversibleMove::Capture(capture) => capture.march(board),
            IrreversibleMove::Pawn(pawn) => pawn.march(board),
            IrreversibleMove::Castle(castle) => castle.march(board),
        }
    }
}

impl IrreversibleMoveExt for IrreversibleMove {}
