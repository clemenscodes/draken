use crate::moves::{coordinates::Coordinates, MoveExt};

use super::IrreversibleMoveExt;

#[derive(Debug)]
pub struct CaptureMove {
    coordinates: Coordinates,
}

impl CaptureMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait CaptureMoveExt: IrreversibleMoveExt {}

impl CaptureMoveExt for CaptureMove {}
impl IrreversibleMoveExt for CaptureMove {}
impl MoveExt for CaptureMove {}
