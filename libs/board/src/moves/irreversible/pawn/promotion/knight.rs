use crate::{
    moves::{
        coordinates::Coordinates,
        irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
        Encode, MoveExt,
    },
    Board,
};

use super::PromotionMoveExt;
use api::Square;
use std::fmt::{Debug, Display};

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

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        self.promote(board)?;
        Ok(())
    }
}

impl Encode for KnightPromotionMove {}

impl Display for KnightPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for KnightPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
