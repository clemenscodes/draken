use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    Encode, MoveExt,
};

use super::PromotionMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BishopPromotionMove {
    coordinates: Coordinates,
}

impl BishopPromotionMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
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
}

impl Encode for BishopPromotionMove {}
