use crate::{coordinates::Coordinates, irreversible::IrreversibleMoveExt, MoveExt};

use super::PawnMoveExt;

#[derive(Debug)]
pub struct EnPassantMove {
    coordinates: Coordinates,
}

impl EnPassantMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait EnPassantMoveExt: PawnMoveExt {}

impl EnPassantMoveExt for EnPassantMove {}
impl PawnMoveExt for EnPassantMove {}
impl IrreversibleMoveExt for EnPassantMove {}
impl MoveExt for EnPassantMove {}
