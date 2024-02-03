mod capture;
mod enpassant;
mod promotion;
mod push;

use capture::PawnCaptureMove;
use enpassant::EnPassantMove;
use promotion::PromotionMove;
use push::PushMove;

use crate::MoveExt;

use super::IrreversibleMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PawnMove {
    Push(PushMove),
    Capture(PawnCaptureMove),
    EnPassant(EnPassantMove),
    Promotion(PromotionMove),
}

impl From<PromotionMove> for PawnMove {
    fn from(v: PromotionMove) -> Self {
        Self::Promotion(v)
    }
}

impl From<EnPassantMove> for PawnMove {
    fn from(v: EnPassantMove) -> Self {
        Self::EnPassant(v)
    }
}

impl From<PawnCaptureMove> for PawnMove {
    fn from(v: PawnCaptureMove) -> Self {
        Self::Capture(v)
    }
}

impl From<PushMove> for PawnMove {
    fn from(v: PushMove) -> Self {
        Self::Push(v)
    }
}

pub trait PawnMoveExt: IrreversibleMoveExt {}

impl PawnMoveExt for PawnMove {}
impl IrreversibleMoveExt for PawnMove {}
impl MoveExt for PawnMove {}
