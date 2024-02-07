pub mod quiet;

use super::MoveExt;
use crate::coordinates::Coordinates;
use board::Board;
use quiet::QuietMove;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReversibleMove {
    Quiet(QuietMove),
}

pub trait ReversibleMoveExt: MoveExt {}

impl ReversibleMoveExt for ReversibleMove {}

impl MoveExt for ReversibleMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            ReversibleMove::Quiet(quiet) => quiet.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) {
        match *self {
            ReversibleMove::Quiet(quiet) => quiet.march(board),
        }
    }
}
