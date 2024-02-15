use super::PromotionMoveExt;
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
pub struct BishopPromotionMove {
    coordinates: Coordinates,
}

impl BishopPromotionMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait BishopPromotionMoveExt: PromotionMoveExt {}

impl MoveExt for BishopPromotionMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.promote(board)
    }
}

impl Display for BishopPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for BishopPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T: PromotionMoveExt> BishopPromotionMoveExt for T {}
