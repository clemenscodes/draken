pub(crate) mod bishop;
pub(crate) mod capture;
pub(crate) mod knight;
pub(crate) mod queen;
pub(crate) mod rook;

use bishop::BishopPromotionMove;
use capture::PromotionCaptureMove;
use knight::KnightPromotionMove;
use queen::QueenPromotionMove;
use rook::RookPromotionMove;

use crate::{irreversible::IrreversibleMoveExt, MoveExt};

use super::PawnMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionMove {
    PromotionCapture(PromotionCaptureMove),
    Queen(QueenPromotionMove),
    Rook(RookPromotionMove),
    Knight(KnightPromotionMove),
    Bishop(BishopPromotionMove),
}

impl From<BishopPromotionMove> for PromotionMove {
    fn from(v: BishopPromotionMove) -> Self {
        Self::Bishop(v)
    }
}

impl From<KnightPromotionMove> for PromotionMove {
    fn from(v: KnightPromotionMove) -> Self {
        Self::Knight(v)
    }
}

impl From<RookPromotionMove> for PromotionMove {
    fn from(v: RookPromotionMove) -> Self {
        Self::Rook(v)
    }
}

impl From<QueenPromotionMove> for PromotionMove {
    fn from(v: QueenPromotionMove) -> Self {
        Self::Queen(v)
    }
}

impl From<PromotionCaptureMove> for PromotionMove {
    fn from(v: PromotionCaptureMove) -> Self {
        Self::PromotionCapture(v)
    }
}

pub trait PromotionMoveExt: PawnMoveExt {}

impl PromotionMoveExt for PromotionMove {}
impl PawnMoveExt for PromotionMove {}
impl IrreversibleMoveExt for PromotionMove {}

impl MoveExt for PromotionMove {
    fn coordinates(&self) -> crate::coordinates::Coordinates {
        match *self {
            PromotionMove::PromotionCapture(promotion) => promotion.coordinates(),
            PromotionMove::Queen(queen) => queen.coordinates(),
            PromotionMove::Rook(rook) => rook.coordinates(),
            PromotionMove::Knight(knight) => knight.coordinates(),
            PromotionMove::Bishop(bishop) => bishop.coordinates(),
        }
    }
}
