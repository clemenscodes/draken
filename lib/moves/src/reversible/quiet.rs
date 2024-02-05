use std::fmt::{Debug, Display};

use api::Square;

use crate::{coordinates::Coordinates, Encode, MoveExt};

use super::ReversibleMoveExt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QuietMove {
    coordinates: Coordinates,
}

impl QuietMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QuietMoveExt: ReversibleMoveExt {}

impl QuietMoveExt for QuietMove {}
impl ReversibleMoveExt for QuietMove {}

impl MoveExt for QuietMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for QuietMove {}

impl Display for QuietMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for QuietMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
