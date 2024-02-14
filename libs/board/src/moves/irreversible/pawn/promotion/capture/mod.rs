pub mod bishop;
pub mod knight;
pub mod queen;
pub mod rook;

use super::PromotionMoveExt;
use crate::{
    moves::{
        coordinates::Coordinates,
        irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
        MoveExt,
    },
    Board,
};
use bishop::BishopPromotionCaptureMove;
use knight::KnightPromotionCaptureMove;
use queen::QueenPromotionCaptureMove;
use rook::RookPromotionCaptureMove;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionCaptureMove {
    Queen(QueenPromotionCaptureMove),
    Rook(RookPromotionCaptureMove),
    Knight(KnightPromotionCaptureMove),
    Bishop(BishopPromotionCaptureMove),
}

pub trait PromotionCaptureMoveExt: PromotionMoveExt {
    fn capture(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.promote(board)
    }
}

impl MoveExt for PromotionCaptureMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            PromotionCaptureMove::Queen(queen) => queen.coordinates(),
            PromotionCaptureMove::Rook(rook) => rook.coordinates(),
            PromotionCaptureMove::Knight(knight) => knight.coordinates(),
            PromotionCaptureMove::Bishop(bishop) => bishop.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        match *self {
            PromotionCaptureMove::Queen(queen) => queen.march(board),
            PromotionCaptureMove::Rook(rook) => rook.march(board),
            PromotionCaptureMove::Knight(knight) => knight.march(board),
            PromotionCaptureMove::Bishop(bishop) => bishop.march(board),
        }
    }
}

impl PromotionCaptureMoveExt for PromotionCaptureMove {}
impl PromotionMoveExt for PromotionCaptureMove {}
impl PawnMoveExt for PromotionCaptureMove {}
impl IrreversibleMoveExt for PromotionCaptureMove {}
