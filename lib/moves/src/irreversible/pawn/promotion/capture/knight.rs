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
pub struct KnightPromotionCaptureMove {
    coordinates: Coordinates,
}

impl KnightPromotionCaptureMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait KnightPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

impl KnightPromotionCaptureMoveExt for KnightPromotionCaptureMove {}
impl PromotionCaptureMoveExt for KnightPromotionCaptureMove {}
impl PromotionMoveExt for KnightPromotionCaptureMove {}
impl PawnMoveExt for KnightPromotionCaptureMove {}
impl IrreversibleMoveExt for KnightPromotionCaptureMove {}

impl MoveExt for KnightPromotionCaptureMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for KnightPromotionCaptureMove {}

impl Display for KnightPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "knight promotion capture move")
    }
}

impl Debug for KnightPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
