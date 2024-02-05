use std::fmt::{Debug, Display};

use api::Square;

use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    Encode, MoveExt,
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

impl DoublePushMoveExt for DoublePushMove {}
impl PawnMoveExt for DoublePushMove {}
impl IrreversibleMoveExt for DoublePushMove {}

impl MoveExt for DoublePushMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for DoublePushMove {}

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