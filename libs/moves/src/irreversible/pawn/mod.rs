pub(crate) mod enpassant;
pub(crate) mod promotion;
pub(crate) mod push;

use enpassant::EnPassantMove;
use promotion::PromotionMove;
use push::DoublePushMove;

use crate::MoveExt;

use super::IrreversibleMoveExt;

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
    fn coordinates(&self) -> crate::coordinates::Coordinates {
        match *self {
            PawnMove::Push(push) => push.coordinates(),
            PawnMove::EnPassant(enpassant) => enpassant.coordinates(),
            PawnMove::Promotion(promotion) => promotion.coordinates(),
        }
    }
}
