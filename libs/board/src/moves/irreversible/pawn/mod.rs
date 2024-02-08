pub mod enpassant;
pub mod promotion;
pub mod push;

use crate::{
    moves::{coordinates::Coordinates, MoveExt},
    Board,
};

use super::IrreversibleMoveExt;
use enpassant::EnPassantMove;
use promotion::PromotionMove;
use push::DoublePushMove;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PawnMove {
    Push(DoublePushMove),
    EnPassant(EnPassantMove),
    Promotion(PromotionMove),
}

pub trait PawnMoveExt: IrreversibleMoveExt {}

impl PawnMoveExt for PawnMove {}
impl IrreversibleMoveExt for PawnMove {}

impl MoveExt for PawnMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            PawnMove::Push(push) => push.coordinates(),
            PawnMove::EnPassant(enpassant) => enpassant.coordinates(),
            PawnMove::Promotion(promotion) => promotion.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) {
        match *self {
            PawnMove::Push(push) => push.march(board),
            PawnMove::EnPassant(enpassant) => enpassant.march(board),
            PawnMove::Promotion(promotion) => promotion.march(board),
        }
    }
}
