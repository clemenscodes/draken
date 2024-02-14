pub mod quiet;

use std::error::Error;

use super::{coordinates::Coordinates, MoveExt};
use crate::{fen::half_move_clock::HalfMoveClockExt, Board};
use quiet::QuietMove;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReversibleMove {
    Quiet(QuietMove),
}

pub trait ReversibleMoveExt: MoveExt {
    fn increment_half_move_clock(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.switch(board)?;
        board.fen_mut().half_move_clock_mut().increment();
        Ok(())
    }
}

impl ReversibleMoveExt for ReversibleMove {}

impl MoveExt for ReversibleMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            ReversibleMove::Quiet(quiet) => quiet.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        match *self {
            ReversibleMove::Quiet(quiet) => quiet.march(board),
        }
    }
}
