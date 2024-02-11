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
pub struct QueenPromotionCaptureMove {
    coordinates: Coordinates,
}

impl QueenPromotionCaptureMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
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

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        self.capture(board)?;
        Ok(())
    }
}

impl Encode for QueenPromotionCaptureMove {}

impl Display for QueenPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for QueenPromotionCaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
