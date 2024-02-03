use crate::{
    coordinates::Coordinates,
    irreversible::{
        pawn::{promotion::PromotionMoveExt, PawnMoveExt},
        IrreversibleMoveExt,
    },
    MoveExt,
};

use super::PromotionCaptureMoveExt;

#[derive(Debug)]
pub struct BishopPromotionCaptureMove {
    coordinates: Coordinates,
}

impl BishopPromotionCaptureMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait BishopPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

impl BishopPromotionCaptureMoveExt for BishopPromotionCaptureMove {}
impl PromotionCaptureMoveExt for BishopPromotionCaptureMove {}
impl PromotionMoveExt for BishopPromotionCaptureMove {}
impl PawnMoveExt for BishopPromotionCaptureMove {}
impl IrreversibleMoveExt for BishopPromotionCaptureMove {}
impl MoveExt for BishopPromotionCaptureMove {}
