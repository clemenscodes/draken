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
pub struct KnightPromotionCaptureMove {
    coordinates: Coordinates,
}

impl KnightPromotionCaptureMove {
    pub fn new(coordinates: Coordinates) -> Self {
        Self { coordinates }
    }

    pub fn coordinates(&self) -> &Coordinates {
        &self.coordinates
    }
}

pub trait KnightPromotionCaptureMoveExt: PromotionCaptureMoveExt {}

impl KnightPromotionCaptureMoveExt for KnightPromotionCaptureMove {}
impl PromotionCaptureMoveExt for KnightPromotionCaptureMove {}
impl PromotionMoveExt for KnightPromotionCaptureMove {}
impl PawnMoveExt for KnightPromotionCaptureMove {}
impl IrreversibleMoveExt for KnightPromotionCaptureMove {}
impl MoveExt for KnightPromotionCaptureMove {}
