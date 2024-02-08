use crate::{
    moves::{
        coordinates::Coordinates,
        irreversible::{
            pawn::{promotion::PromotionMoveExt, PawnMoveExt},
            IrreversibleMoveExt,
        },
        Encode, MoveExt,
    },
    Board,
};

use super::PromotionCaptureMoveExt;
use api::Square;
use std::fmt::{Debug, Display};

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

    fn march(&self, board: &mut Board) {
        todo!()
    }
}

impl Encode for KnightPromotionCaptureMove {}

impl Display for KnightPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for KnightPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
