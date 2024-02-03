use crate::{
    coordinates::Coordinates,
    irreversible::{
        pawn::{promotion::PromotionMoveExt, PawnMoveExt},
        IrreversibleMoveExt,
    },
    MoveExt,
};

use super::PromotionCaptureMoveExt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RookPromotionCaptureMove {
    coordinates: Coordinates,
}

impl RookPromotionCaptureMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait RookPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

impl RookPromotionCaptureMoveExt for RookPromotionCaptureMove {}
impl PromotionCaptureMoveExt for RookPromotionCaptureMove {}
impl PromotionMoveExt for RookPromotionCaptureMove {}
impl PawnMoveExt for RookPromotionCaptureMove {}
impl IrreversibleMoveExt for RookPromotionCaptureMove {}
impl MoveExt for RookPromotionCaptureMove {}
