use crate::{coordinates::Coordinates, Encode, MoveExt};

use super::IrreversibleMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureMove {
    coordinates: Coordinates,
}

impl CaptureMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
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
