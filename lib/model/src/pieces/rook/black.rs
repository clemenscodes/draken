use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::RookExt;

#[derive(Debug)]
pub struct BlackRook {
    bitboard: Bitboard,
}

impl BlackRook {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait BlackRookExt: RookExt {}

impl BlackRookExt for BlackRook {}
impl RookExt for BlackRook {}
impl PieceExt for BlackRook {}
