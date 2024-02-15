use super::PromotionCaptureMoveExt;
use crate::{
    moves::{coordinates::Coordinates, MoveExt},
    Board,
};
use api::Square;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RookPromotionCaptureMove {
    coordinates: Coordinates,
}

pub trait RookPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

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

impl MoveExt for RookPromotionCaptureMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.capture(board)
    }
}

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

impl<T: PromotionCaptureMoveExt> RookPromotionCaptureMoveExt for T {}
