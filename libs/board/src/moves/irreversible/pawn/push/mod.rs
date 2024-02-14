use super::PawnMoveExt;
use crate::{
    moves::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, Encode, MoveExt},
    Board,
};
use api::Square;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DoublePushMove {
    coordinates: Coordinates,
}

impl DoublePushMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait DoublePushMoveExt: PawnMoveExt {}

impl MoveExt for DoublePushMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.push(self.coordinates().source(), board)
    }
}

impl Display for DoublePushMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for DoublePushMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Encode for DoublePushMove {}
impl DoublePushMoveExt for DoublePushMove {}
impl PawnMoveExt for DoublePushMove {}
impl IrreversibleMoveExt for DoublePushMove {}
