use std::fmt::{Debug, Display};

use api::Square;

use crate::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, Encode, MoveExt};

use super::CastleMoveExt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QueenCastleMove {
    coordinates: Coordinates,
}

impl QueenCastleMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
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

impl Display for QueenCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "queen castle move")
    }
}

impl Debug for QueenCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
