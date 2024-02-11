pub mod king;
pub mod queen;

use super::IrreversibleMoveExt;
use crate::{
    moves::{coordinates::Coordinates, MoveExt},
    Board,
};
use king::KingCastleMove;
use queen::QueenCastleMove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleMove {
    King(KingCastleMove),
    Queen(QueenCastleMove),
}

pub trait CastleMoveExt: IrreversibleMoveExt {
    fn castle(&self, board: &mut Board) -> Result<(), ()> {
        self.make(board)
    }
}

impl CastleMoveExt for CastleMove {}
impl IrreversibleMoveExt for CastleMove {}

impl MoveExt for CastleMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            CastleMove::King(king) => king.coordinates(),
            CastleMove::Queen(queen) => queen.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        match *self {
            CastleMove::King(king) => king.march(board),
            CastleMove::Queen(queen) => queen.march(board),
        }
    }
}
