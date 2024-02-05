use std::fmt::{Debug, Display};

use api::Square;

use crate::{
    coordinates::Coordinates,
    irreversible::{
        pawn::{promotion::PromotionMoveExt, PawnMoveExt},
        IrreversibleMoveExt,
    },
    Encode, MoveExt,
};

use super::PromotionCaptureMoveExt;

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
}

impl Encode for BishopPromotionCaptureMove {}

impl Display for BishopPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bishop promotion capture move")
    }
}

impl Debug for BishopPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
