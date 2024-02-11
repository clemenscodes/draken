use crate::{
    moves::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, Encode, MoveExt},
    Board,
};

use super::CastleMoveExt;
use api::Square;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KingCastleMove {
    coordinates: Coordinates,
}

impl KingCastleMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
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

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        todo!()
    }
}

impl Encode for KingCastleMove {}

impl Display for KingCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for KingCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
