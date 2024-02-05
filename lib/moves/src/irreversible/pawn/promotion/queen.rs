use std::fmt::{Debug, Display};

use api::Square;

use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    Encode, MoveExt,
};

use super::PromotionMoveExt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct QueenPromotionMove {
    coordinates: Coordinates,
}

impl QueenPromotionMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QueenPromotionMoveExt: PromotionMoveExt {}

impl QueenPromotionMoveExt for QueenPromotionMove {}
impl PromotionMoveExt for QueenPromotionMove {}
impl PawnMoveExt for QueenPromotionMove {}
impl IrreversibleMoveExt for QueenPromotionMove {}

impl MoveExt for QueenPromotionMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for QueenPromotionMove {}

impl Display for QueenPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "queen promotion move")
    }
}

impl Debug for QueenPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
