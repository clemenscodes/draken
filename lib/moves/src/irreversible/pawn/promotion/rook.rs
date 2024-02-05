use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    Encode, MoveExt,
};

use super::PromotionMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RookPromotionMove {
    coordinates: Coordinates,
}

impl RookPromotionMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait RookPromotionMoveExt: PromotionMoveExt {}

impl RookPromotionMoveExt for RookPromotionMove {}
impl PromotionMoveExt for RookPromotionMove {}
impl PawnMoveExt for RookPromotionMove {}
impl IrreversibleMoveExt for RookPromotionMove {}

impl MoveExt for RookPromotionMove {
    fn coordinates(&self) -> Coordinates {
        *self.coordinates()
    }
}

impl Encode for RookPromotionMove {}
