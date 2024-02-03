mod king;
mod queen;

use king::KingCastleMove;
use queen::QueenCastleMove;

use crate::MoveExt;

use super::IrreversibleMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastleMove {
    King(KingCastleMove),
    Queen(QueenCastleMove),
}

impl From<QueenCastleMove> for CastleMove {
    fn from(v: QueenCastleMove) -> Self {
        Self::Queen(v)
    }
}

impl From<KingCastleMove> for CastleMove {
    fn from(v: KingCastleMove) -> Self {
        Self::King(v)
    }
}

pub trait CastleMoveExt: IrreversibleMoveExt {}

impl CastleMoveExt for CastleMove {}
impl IrreversibleMoveExt for CastleMove {}
impl MoveExt for CastleMove {}
