use crate::moves::{
    coordinates::Coordinates,
    irreversible::{pawn::PawnMoveExt, IrreversibleMoveExt},
    MoveExt,
};

use super::PromotionMoveExt;

#[derive(Debug)]
pub struct QueenPromotionMove {
    coordinates: Coordinates,
}

impl QueenPromotionMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait QueenPromotionMoveExt: PromotionMoveExt {}

impl QueenPromotionMoveExt for QueenPromotionMove {}
impl PromotionMoveExt for QueenPromotionMove {}
impl PawnMoveExt for QueenPromotionMove {}
impl IrreversibleMoveExt for QueenPromotionMove {}
impl MoveExt for QueenPromotionMove {}
