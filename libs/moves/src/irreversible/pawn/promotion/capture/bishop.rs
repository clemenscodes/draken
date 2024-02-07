use super::PromotionCaptureMoveExt;
use crate::{
    coordinates::Coordinates,
    irreversible::{
        pawn::{promotion::PromotionMoveExt, PawnMoveExt},
        IrreversibleMoveExt,
    },
    Encode, MoveExt,
};
use api::Square;
use board::Board;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BishopPromotionCaptureMove {
    coordinates: Coordinates,
}

impl BishopPromotionCaptureMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait BishopPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

impl BishopPromotionCaptureMoveExt for BishopPromotionCaptureMove {}
impl PromotionCaptureMoveExt for BishopPromotionCaptureMove {}
impl PromotionMoveExt for BishopPromotionCaptureMove {}
impl PawnMoveExt for BishopPromotionCaptureMove {}
impl IrreversibleMoveExt for BishopPromotionCaptureMove {}

impl MoveExt for BishopPromotionCaptureMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) {
        todo!()
    }
}

impl Encode for BishopPromotionCaptureMove {}

impl Display for BishopPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for BishopPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
