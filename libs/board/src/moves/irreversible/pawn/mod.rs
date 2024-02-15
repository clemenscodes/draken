pub mod enpassant;
pub mod promotion;
pub mod push;

use super::IrreversibleMoveExt;
use crate::{
    moves::{coordinates::Coordinates, MoveExt},
    Board,
};
use api::Square;
use bitboard::BitboardExt;
use enpassant::EnPassantMove;
use promotion::PromotionMove;
use push::DoublePushMove;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PawnMove {
    Push(DoublePushMove),
    EnPassant(EnPassantMove),
    Promotion(PromotionMove),
}

pub trait PawnMoveExt: IrreversibleMoveExt {
    fn push(&self, source: Square, board: &mut Board) -> Result<(), Box<dyn Error>> {
        self.make(board)?;
        let piece = board.get_piece_board_mut(source)?;
        piece.self_unset_bit(source.into());
        Ok(())
    }
}

impl MoveExt for PawnMove {
    fn coordinates(&self) -> Coordinates {
        match *self {
            PawnMove::Push(push) => push.coordinates(),
            PawnMove::EnPassant(enpassant) => enpassant.coordinates(),
            PawnMove::Promotion(promotion) => promotion.coordinates(),
        }
    }

    fn march(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        match *self {
            PawnMove::Push(push) => push.march(board),
            PawnMove::EnPassant(enpassant) => enpassant.march(board),
            PawnMove::Promotion(promotion) => promotion.march(board),
        }
    }
}

impl<T: IrreversibleMoveExt> PawnMoveExt for T {}
