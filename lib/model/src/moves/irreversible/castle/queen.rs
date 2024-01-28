use crate::moves::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, MoveExt};

use super::CastleMoveExt;

#[derive(Debug)]
pub struct QueenCastleMove {
    coordinates: Coordinates,
}

impl QueenCastleMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QueenCastleMoveExt: CastleMoveExt {}

impl QueenCastleMoveExt for QueenCastleMove {}
impl CastleMoveExt for QueenCastleMove {}
impl IrreversibleMoveExt for QueenCastleMove {}
impl MoveExt for QueenCastleMove {}
