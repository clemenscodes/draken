use crate::moves::{coordinates::Coordinates, MoveExt};

use super::ReversibleMoveExt;

#[derive(Debug)]
pub struct QuietMove {
    coordinates: Coordinates,
}

impl QuietMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QuietMoveExt: ReversibleMoveExt {}

impl QuietMoveExt for QuietMove {}
impl ReversibleMoveExt for QuietMove {}
impl MoveExt for QuietMove {}
