use super::CastleMoveExt;
use crate::{
    moves::{coordinates::Coordinates, MoveExt},
    Board,
};
use api::Square;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

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

impl MoveExt for QueenCastleMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.castle(board)
    }
}

impl Display for QueenCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for QueenCastleMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl QueenCastleMoveExt for QueenCastleMove {}
impl CastleMoveExt for QueenCastleMove {}
