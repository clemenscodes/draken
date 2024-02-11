use super::PawnMoveExt;
use crate::{
    moves::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, Encode, MoveExt},
    Board,
};
use api::Square;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EnPassantMove {
    coordinates: Coordinates,
}

impl EnPassantMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait EnPassantMoveExt: PawnMoveExt {}

impl EnPassantMoveExt for EnPassantMove {}
impl PawnMoveExt for EnPassantMove {}
impl IrreversibleMoveExt for EnPassantMove {}

impl MoveExt for EnPassantMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        todo!()
    }
}

impl Encode for EnPassantMove {}

impl Display for EnPassantMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for EnPassantMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
