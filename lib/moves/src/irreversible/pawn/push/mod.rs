use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    Encode, MoveExt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoublePushMove {
    coordinates: Coordinates,
}

impl DoublePushMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
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
