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
use std::fmt::{Debug, Display};

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

impl BishopPromotionMoveExt for BishopPromotionMove {}
impl PromotionMoveExt for BishopPromotionMove {}
impl PawnMoveExt for BishopPromotionMove {}
impl IrreversibleMoveExt for BishopPromotionMove {}

impl MoveExt for BishopPromotionMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        self.promote(board)?;
        Ok(())
    }
}

impl Encode for BishopPromotionMove {}

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
