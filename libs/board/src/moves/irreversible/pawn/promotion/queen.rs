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

    fn march(&self, board: &mut Board) -> Result<(), ()> {
        self.promote(board)
    }
}

impl Encode for QueenPromotionMove {}

impl Display for QueenPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let source = self.coordinates().source();
        let destination = self.coordinates().destination();
        write!(f, "{source}{destination}")
    }
}

impl Debug for QueenPromotionMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
