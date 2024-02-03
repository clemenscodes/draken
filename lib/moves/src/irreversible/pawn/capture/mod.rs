use crate::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, MoveExt};

use super::PawnMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PawnCaptureMove {
    coordinates: Coordinates,
}

impl PawnCaptureMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait PawnCaptureMoveExt: PawnMoveExt {}

impl PawnCaptureMoveExt for PawnCaptureMove {}
impl PawnMoveExt for PawnCaptureMove {}
impl IrreversibleMoveExt for PawnCaptureMove {}
impl MoveExt for PawnCaptureMove {}
