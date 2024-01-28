use crate::{bitboard::Bitboard, pieces::PieceExt};

use super::QueenExt;

#[derive(Debug)]
pub struct WhiteQueen {
    bitboard: Bitboard,
}

impl WhiteQueen {
    pub fn new(bitboard: Bitboard) -> Self {
        Self { bitboard }
    }

    pub fn bitboard(&self) -> Bitboard {
        self.bitboard
    }
}

pub trait WhiteQueenExt: QueenExt {}

impl WhiteQueenExt for WhiteQueen {}
impl QueenExt for WhiteQueen {}
impl PieceExt for WhiteQueen {}
