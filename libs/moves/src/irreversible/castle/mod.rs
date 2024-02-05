pub(crate) mod king;
pub(crate) mod queen;

use king::KingCastleMove;
use queen::QueenCastleMove;

use crate::MoveExt;

use super::IrreversibleMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleMove {
    King(KingCastleMove),
    Queen(QueenCastleMove),
}

pub trait CastleMoveExt: IrreversibleMoveExt {}

impl CastleMoveExt for CastleMove {}
impl IrreversibleMoveExt for CastleMove {}

impl MoveExt for CastleMove {
    fn coordinates(&self) -> crate::coordinates::Coordinates {
        match *self {
            CastleMove::King(king) => king.coordinates(),
            CastleMove::Queen(queen) => queen.coordinates(),
        }
    }
}
