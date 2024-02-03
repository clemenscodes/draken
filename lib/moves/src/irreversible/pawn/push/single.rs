use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    MoveExt,
};

use super::PushMoveExt;

#[derive(Debug)]
pub struct SinglePushMove {
    coordinates: Coordinates,
}

impl SinglePushMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait SinglePushMoveExt: PushMoveExt {}

impl SinglePushMoveExt for SinglePushMove {}
impl PushMoveExt for SinglePushMove {}
impl PawnMoveExt for SinglePushMove {}
impl IrreversibleMoveExt for SinglePushMove {}
impl MoveExt for SinglePushMove {}
