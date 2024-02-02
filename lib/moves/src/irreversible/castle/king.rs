use crate::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, MoveExt};

use super::CastleMoveExt;

#[derive(Debug)]
pub struct KingCastleMove {
    coordinates: Coordinates,
}

impl KingCastleMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait KingCastleMoveExt: CastleMoveExt {}

impl KingCastleMoveExt for KingCastleMove {}
impl CastleMoveExt for KingCastleMove {}
impl IrreversibleMoveExt for KingCastleMove {}
impl MoveExt for KingCastleMove {}
