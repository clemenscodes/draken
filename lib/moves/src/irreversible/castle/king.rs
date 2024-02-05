use std::fmt::{Debug, Display};

use api::Square;

use crate::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, Encode, MoveExt};

use super::CastleMoveExt;

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
}

impl Encode for KingCastleMove {}

impl Display for KingCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "king castle move")
    }
}

impl Debug for KingCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
