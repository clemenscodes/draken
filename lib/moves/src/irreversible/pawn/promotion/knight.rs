use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    Encode, MoveExt,
};

use super::PromotionMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KnightPromotionMove {
    coordinates: Coordinates,
}

impl KnightPromotionMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
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
