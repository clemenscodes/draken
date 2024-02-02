use crate::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    MoveExt,
};

use super::PromotionMoveExt;

#[derive(Debug)]
pub struct BishopPromotionMove {
    coordinates: Coordinates,
}

impl BishopPromotionMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait BishopPromotionMoveExt: PromotionMoveExt {}

impl BishopPromotionMoveExt for BishopPromotionMove {}
impl PromotionMoveExt for BishopPromotionMove {}
impl PawnMoveExt for BishopPromotionMove {}
impl IrreversibleMoveExt for BishopPromotionMove {}
impl MoveExt for BishopPromotionMove {}
