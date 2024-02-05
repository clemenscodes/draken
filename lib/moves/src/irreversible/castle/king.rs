use crate::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, Encode, MoveExt};

use super::CastleMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KingCastleMove {
    coordinates: Coordinates,
}

impl KingCastleMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait KingCastleMoveExt: CastleMoveExt {}

impl KingCastleMoveExt for KingCastleMove {}
impl CastleMoveExt for KingCastleMove {}
impl IrreversibleMoveExt for KingCastleMove {}

impl MoveExt for KingCastleMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for KingCastleMove {}
