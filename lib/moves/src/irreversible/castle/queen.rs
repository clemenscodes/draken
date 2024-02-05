use crate::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, Encode, MoveExt};

use super::CastleMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueenCastleMove {
    coordinates: Coordinates,
}

impl QueenCastleMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QueenCastleMoveExt: CastleMoveExt {}

impl QueenCastleMoveExt for QueenCastleMove {}
impl CastleMoveExt for QueenCastleMove {}
impl IrreversibleMoveExt for QueenCastleMove {}

impl MoveExt for QueenCastleMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for QueenCastleMove {}
