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
