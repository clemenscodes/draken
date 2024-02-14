use super::PromotionMoveExt;
use crate::{
    moves::{
        coordinates::Coordinates,
        irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
        Encode, MoveExt,
    },
    Board,
};
use api::Square;
use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RookPromotionMove {
    coordinates: Coordinates,
}

impl RookPromotionMove {
    pub fn new(source: Square, destination: Square) -> Self {
        Self {
            coordinates: Coordinates::new(source, destination),
        }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait RookPromotionMoveExt: PromotionMoveExt {}

impl MoveExt for RookPromotionMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.promote(board)
    }
}

impl Display for RookPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for RookPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Encode for RookPromotionMove {}
impl RookPromotionMoveExt for RookPromotionMove {}
impl PromotionMoveExt for RookPromotionMove {}
impl PawnMoveExt for RookPromotionMove {}
impl IrreversibleMoveExt for RookPromotionMove {}
