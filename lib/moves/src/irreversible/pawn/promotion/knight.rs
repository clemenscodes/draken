use std::fmt::{Debug, Display};

use api::Square;

use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    Encode, MoveExt,
};

use super::PromotionMoveExt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct KnightPromotionMove {
    coordinates: Coordinates,
}

impl KnightPromotionMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait KnightPromotionMoveExt: PromotionMoveExt {}

impl KnightPromotionMoveExt for KnightPromotionMove {}
impl PromotionMoveExt for KnightPromotionMove {}
impl PawnMoveExt for KnightPromotionMove {}
impl IrreversibleMoveExt for KnightPromotionMove {}

impl MoveExt for KnightPromotionMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for KnightPromotionMove {}

impl Display for KnightPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "knight promotion move")
    }
}

impl Debug for KnightPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
