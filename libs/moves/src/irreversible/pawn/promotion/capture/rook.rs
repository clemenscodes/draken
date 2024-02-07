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

    fn march(&self, board: &mut Board) {
        todo!()
    }
}

impl Encode for RookPromotionCaptureMove {}

impl Display for RookPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for RookPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
