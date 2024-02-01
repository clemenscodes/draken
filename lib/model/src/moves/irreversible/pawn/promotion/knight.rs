use crate::moves::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    MoveExt,
};

use super::PromotionMoveExt;

#[derive(Debug)]
pub struct KnightPromotionMove {
    coordinates: Coordinates,
}

impl KnightPromotionMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait KnightPromotionMoveExt: PromotionMoveExt {}

impl KnightPromotionMoveExt for KnightPromotionMove {}
impl PromotionMoveExt for KnightPromotionMove {}
impl PawnMoveExt for KnightPromotionMove {}
impl IrreversibleMoveExt for KnightPromotionMove {}
impl MoveExt for KnightPromotionMove {}
