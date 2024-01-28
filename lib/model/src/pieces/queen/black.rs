use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::QueenExt;

#[derive(Debug)]
pub struct BlackQueen {
    bitboard: Bitboard,
}

impl BlackQueen {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait BlackQueenExt: QueenExt {}

impl BlackQueenExt for BlackQueen {}
impl QueenExt for BlackQueen {}
impl PieceExt for BlackQueen {}
