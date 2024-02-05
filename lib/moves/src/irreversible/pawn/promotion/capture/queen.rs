use crate::{
    coordinates::Coordinates,
    irreversible::{
        pawn::{promotion::PromotionMoveExt, PawnMoveExt},
        IrreversibleMoveExt,
    },
    Encode, MoveExt,
};

use super::PromotionCaptureMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueenPromotionCaptureMove {
    coordinates: Coordinates,
}

impl QueenPromotionCaptureMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QueenPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

impl QueenPromotionCaptureMoveExt for QueenPromotionCaptureMove {}
impl PromotionCaptureMoveExt for QueenPromotionCaptureMove {}
impl PromotionMoveExt for QueenPromotionCaptureMove {}
impl PawnMoveExt for QueenPromotionCaptureMove {}
impl IrreversibleMoveExt for QueenPromotionCaptureMove {}

impl MoveExt for QueenPromotionCaptureMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for QueenPromotionCaptureMove {}
