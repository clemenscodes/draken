use std::fmt::{Debug, Display};

use api::Square;

use crate::{coordinates::Coordinates, Encode, MoveExt};

use super::IrreversibleMoveExt;

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
}

impl Encode for CaptureMove {}

impl Display for CaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "capture move")
    }
}

impl Debug for CaptureMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
