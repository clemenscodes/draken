use crate::{
    moves::{coordinates::Coordinates, Encode, MoveExt},
    Board,
};

use super::IrreversibleMoveExt;
use api::Square;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CaptureMove {
    coordinates: Coordinates,
}

impl CaptureMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait CaptureMoveExt: IrreversibleMoveExt {}

impl CaptureMoveExt for CaptureMove {}
impl IrreversibleMoveExt for CaptureMove {}

impl MoveExt for CaptureMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) {
        todo!()
    }
}

impl Encode for CaptureMove {}

impl Display for CaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for CaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
