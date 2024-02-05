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
pub struct RookPromotionCaptureMove {
    coordinates: Coordinates,
}

impl RookPromotionCaptureMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait RookPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

impl RookPromotionCaptureMoveExt for RookPromotionCaptureMove {}
impl PromotionCaptureMoveExt for RookPromotionCaptureMove {}
impl PromotionMoveExt for RookPromotionCaptureMove {}
impl PawnMoveExt for RookPromotionCaptureMove {}
impl IrreversibleMoveExt for RookPromotionCaptureMove {}

impl MoveExt for RookPromotionCaptureMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for RookPromotionCaptureMove {}

impl Display for RookPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rook promotion capture move")
    }
}

impl Debug for RookPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
