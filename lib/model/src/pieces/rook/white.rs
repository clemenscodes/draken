use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::RookExt;

#[derive(Debug)]
pub struct WhiteRook {
    bitboard: Bitboard,
}

impl WhiteRook {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait WhiteRookExt: RookExt {}

impl WhiteRookExt for WhiteRook {}
impl RookExt for WhiteRook {}
impl PieceExt for WhiteRook {}
