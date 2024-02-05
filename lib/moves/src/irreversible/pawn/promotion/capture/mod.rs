pub(crate) mod bishop;
pub(crate) mod knight;
pub(crate) mod queen;
pub(crate) mod rook;

use bishop::BishopPromotionCaptureMove;
use knight::KnightPromotionCaptureMove;
use queen::QueenPromotionCaptureMove;
use rook::RookPromotionCaptureMove;

use crate::{
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    MoveExt,
};

use super::PromotionMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionCaptureMove {
    Queen(QueenPromotionCaptureMove),
    Rook(RookPromotionCaptureMove),
    Knight(KnightPromotionCaptureMove),
    Bishop(BishopPromotionCaptureMove),
}

pub trait PromotionCaptureMoveExt: PromotionMoveExt {}

impl PromotionCaptureMoveExt for PromotionCaptureMove {}
impl PromotionMoveExt for PromotionCaptureMove {}
impl PawnMoveExt for PromotionCaptureMove {}
impl IrreversibleMoveExt for PromotionCaptureMove {}

impl MoveExt for PromotionCaptureMove {
    fn coordinates(&self) -> crate::coordinates::Coordinates {
        match *self {
            PromotionCaptureMove::Queen(queen) => queen.coordinates(),
            PromotionCaptureMove::Rook(rook) => rook.coordinates(),
            PromotionCaptureMove::Knight(knight) => knight.coordinates(),
            PromotionCaptureMove::Bishop(bishop) => bishop.coordinates(),
        }
    }
}
