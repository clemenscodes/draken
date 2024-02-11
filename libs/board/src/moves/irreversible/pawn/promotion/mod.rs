pub mod bishop;
pub mod capture;
pub mod knight;
pub mod queen;
pub mod rook;

use super::PawnMoveExt;
use crate::{
    moves::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, MoveExt},
    Board,
};
use bishop::BishopPromotionMove;
use capture::PromotionCaptureMove;
use knight::KnightPromotionMove;
use queen::QueenPromotionMove;
use rook::RookPromotionMove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionMove {
    PromotionCapture(PromotionCaptureMove),
    Queen(QueenPromotionMove),
    Rook(RookPromotionMove),
    Knight(KnightPromotionMove),
    Bishop(BishopPromotionMove),
}

pub trait PromotionMoveExt: PawnMoveExt {
    fn promote(&self, board: &mut Board) -> Result<(), ()> {
        self.push(self.coordinates().source(), board)?;
        Ok(())
    }
}

impl PromotionMoveExt for PromotionMove {}
impl PawnMoveExt for PromotionMove {}
impl IrreversibleMoveExt for PromotionMove {}

impl MoveExt for PromotionMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            PromotionMove::PromotionCapture(promotion) => promotion.coordinates(),
            PromotionMove::Queen(queen) => queen.coordinates(),
            PromotionMove::Rook(rook) => rook.coordinates(),
            PromotionMove::Knight(knight) => knight.coordinates(),
            PromotionMove::Bishop(bishop) => bishop.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        match *self {
            PromotionMove::PromotionCapture(promotion) => promotion.march(board),
            PromotionMove::Queen(queen) => queen.march(board),
            PromotionMove::Rook(rook) => rook.march(board),
            PromotionMove::Knight(knight) => knight.march(board),
            PromotionMove::Bishop(bishop) => bishop.march(board),
        }
    }
}
